use std::fmt::Debug;
use std::io::Cursor;
use std::str::FromStr;

use mediarepo_core::content_descriptor::encode_content_descriptor;
use sea_orm::prelude::*;
use sea_orm::sea_query::{Expr, Query};
use sea_orm::{Condition, DatabaseConnection, Set};
use sea_orm::{JoinType, QuerySelect};
use tokio::io::{AsyncReadExt, BufReader};

use crate::file_metadata::FileMetadata;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::fs::file_hash_store::FileHashStore;
use mediarepo_core::thumbnailer::{self, Thumbnail as ThumbnailerThumb, ThumbnailSize};
use mediarepo_database::entities::content_descriptor;
use mediarepo_database::entities::content_descriptor_tag;
use mediarepo_database::entities::file;
use mediarepo_database::entities::namespace;
use mediarepo_database::entities::tag;

use crate::tag::Tag;

pub enum FileStatus {
    Imported = 10,
    Archived = 20,
    Deleted = 30,
}

#[derive(Clone)]
pub struct File {
    db: DatabaseConnection,
    model: file::Model,
    content_descriptor: content_descriptor::Model,
}

impl File {
    #[tracing::instrument(level = "trace")]
    pub(crate) fn new(
        db: DatabaseConnection,
        model: file::Model,
        hash: content_descriptor::Model,
    ) -> Self {
        Self {
            db,
            model,
            content_descriptor: hash,
        }
    }

    /// Returns a list of all known stored files
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn all(db: DatabaseConnection) -> RepoResult<Vec<File>> {
        let files: Vec<(file::Model, Option<content_descriptor::Model>)> = file::Entity::find()
            .find_also_related(content_descriptor::Entity)
            .all(&db)
            .await?;
        let files = files
            .into_iter()
            .filter_map(|(f, h)| {
                let h = h?;
                Some(Self::new(db.clone(), f, h))
            })
            .collect();

        Ok(files)
    }

    /// Fetches the file by id
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        if let Some((model, Some(hash))) = file::Entity::find_by_id(id)
            .find_also_related(content_descriptor::Entity)
            .one(&db)
            .await?
        {
            let file = File::new(db, model, hash);
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }

    /// Finds the file by hash
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_cd(db: DatabaseConnection, cd: &[u8]) -> RepoResult<Option<Self>> {
        if let Some((hash, Some(model))) = content_descriptor::Entity::find()
            .filter(content_descriptor::Column::Descriptor.eq(cd))
            .find_also_related(file::Entity)
            .one(&db)
            .await?
        {
            let file = File::new(db, model, hash);
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }

    /// Finds the file by tags
    #[tracing::instrument(level = "debug", skip(db))]
    pub(crate) async fn find_by_tags(
        db: DatabaseConnection,
        tag_ids: Vec<Vec<(i64, bool)>>,
    ) -> RepoResult<Vec<Self>> {
        let main_condition = build_find_filter_conditions(tag_ids);

        let results: Vec<(content_descriptor::Model, Option<file::Model>)> =
            content_descriptor::Entity::find()
                .find_also_related(file::Entity)
                .filter(main_condition)
                .group_by(file::Column::Id)
                .all(&db)
                .await?;
        let files: Vec<Self> = results
            .into_iter()
            .filter_map(|(hash, tag)| Some(Self::new(db.clone(), tag?, hash)))
            .collect();

        Ok(files)
    }

    /// Adds a file with its hash to the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub(crate) async fn add(
        db: DatabaseConnection,
        cd_id: i64,
        mime_type: String,
    ) -> RepoResult<Self> {
        let file = file::ActiveModel {
            cd_id: Set(cd_id),
            mime_type: Set(mime_type),
            ..Default::default()
        };
        let file: file::ActiveModel = file.insert(&db).await?.into();
        let file = Self::by_id(db, file.id.unwrap())
            .await?
            .expect("Inserted file does not exist");

        Ok(file)
    }

    /// Returns the unique identifier of the file
    pub fn id(&self) -> i64 {
        self.model.id
    }

    /// Returns the hash of the file (content identifier)
    pub fn cd(&self) -> &[u8] {
        &self.content_descriptor.descriptor
    }

    /// Returns the encoded content descriptor
    pub fn encoded_cd(&self) -> String {
        encode_content_descriptor(self.cd())
    }

    /// Returns the id of the civ (content identifier value) of the file
    pub fn cd_id(&self) -> i64 {
        self.content_descriptor.id
    }

    /// Returns the mime type of the file
    pub fn mime_type(&self) -> &String {
        &self.model.mime_type
    }

    /// Returns the status of the file
    pub fn status(&self) -> FileStatus {
        match self.model.status {
            10 => FileStatus::Imported,
            20 => FileStatus::Archived,
            30 => FileStatus::Deleted,
            _ => FileStatus::Imported,
        }
    }

    /// Returns the metadata associated with this file
    /// A file MUST always have metadata associated
    pub async fn metadata(&self) -> RepoResult<FileMetadata> {
        FileMetadata::by_id(self.db.clone(), self.model.id)
            .await
            .and_then(|f| f.ok_or_else(|| RepoError::from("missing file metadata")))
    }

    /// Returns the list of tags of the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn tags(&self) -> RepoResult<Vec<Tag>> {
        let tags: Vec<(tag::Model, Option<namespace::Model>)> = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .join(
                JoinType::LeftJoin,
                content_descriptor_tag::Relation::Tag.def().rev(),
            )
            .join(
                JoinType::InnerJoin,
                content_descriptor_tag::Relation::ContentDescriptorId.def(),
            )
            .filter(content_descriptor::Column::Id.eq(self.content_descriptor.id))
            .all(&self.db)
            .await?;
        let tags = tags
            .into_iter()
            .map(|(tag, namespace)| Tag::new(self.db.clone(), tag, namespace))
            .collect();

        Ok(tags)
    }

    /// Adds a single tag to the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_tag(&mut self, tag_id: i64) -> RepoResult<()> {
        let cd_id = self.content_descriptor.id;
        let active_model = content_descriptor_tag::ActiveModel {
            cd_id: Set(cd_id),
            tag_id: Set(tag_id),
        };
        active_model.insert(&self.db).await?;
        Ok(())
    }

    /// Adds multiple tags to the file at once
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_tags(&self, tag_ids: Vec<i64>) -> RepoResult<()> {
        if tag_ids.is_empty() {
            return Ok(());
        }
        let cd_id = self.content_descriptor.id;
        let models: Vec<content_descriptor_tag::ActiveModel> = tag_ids
            .into_iter()
            .map(|tag_id| content_descriptor_tag::ActiveModel {
                cd_id: Set(cd_id),
                tag_id: Set(tag_id),
            })
            .collect();
        content_descriptor_tag::Entity::insert_many(models)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    /// Removes multiple tags from the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn remove_tags(&self, tag_ids: Vec<i64>) -> RepoResult<()> {
        let hash_id = self.content_descriptor.id;
        content_descriptor_tag::Entity::delete_many()
            .filter(content_descriptor_tag::Column::CdId.eq(hash_id))
            .filter(content_descriptor_tag::Column::TagId.is_in(tag_ids))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    /// Returns the reader for the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_reader(
        &self,
        storage: &FileHashStore,
    ) -> RepoResult<BufReader<tokio::fs::File>> {
        storage
            .get_file(&self.content_descriptor.descriptor)
            .await
            .map(|(_, f)| f)
    }

    /// Creates a thumbnail for the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn create_thumbnail<I: IntoIterator<Item = ThumbnailSize> + Debug>(
        &self,
        storage: &FileHashStore,
        sizes: I,
    ) -> RepoResult<Vec<ThumbnailerThumb>> {
        let mut buf = Vec::new();
        self.get_reader(storage)
            .await?
            .read_to_end(&mut buf)
            .await?;
        let mime_type = self.model.mime_type.clone();
        let mime_type =
            mime::Mime::from_str(&mime_type).unwrap_or_else(|_| mime::APPLICATION_OCTET_STREAM);
        let thumbs = thumbnailer::create_thumbnails(Cursor::new(buf), mime_type, sizes)?;

        Ok(thumbs)
    }
}

fn build_find_filter_conditions(tag_ids: Vec<Vec<(i64, bool)>>) -> Condition {
    let mut main_condition = Condition::all();

    for mut expression in tag_ids {
        if expression.len() == 1 {
            let (tag_id, negated) = expression.pop().unwrap();
            main_condition = add_single_filter_expression(main_condition, tag_id, negated)
        } else if !expression.is_empty() {
            let mut sub_condition = Condition::any();

            for (tag, negated) in expression {
                sub_condition = add_single_filter_expression(sub_condition, tag, negated);
            }
            main_condition = main_condition.add(sub_condition);
        }
    }
    main_condition
}

fn add_single_filter_expression(condition: Condition, tag_id: i64, negated: bool) -> Condition {
    if negated {
        condition.add(
            content_descriptor::Column::Id.not_in_subquery(
                Query::select()
                    .expr(Expr::col(content_descriptor_tag::Column::CdId))
                    .from(content_descriptor_tag::Entity)
                    .cond_where(content_descriptor_tag::Column::TagId.eq(tag_id))
                    .to_owned(),
            ),
        )
    } else {
        condition.add(
            content_descriptor::Column::Id.in_subquery(
                Query::select()
                    .expr(Expr::col(content_descriptor_tag::Column::CdId))
                    .from(content_descriptor_tag::Entity)
                    .cond_where(content_descriptor_tag::Column::TagId.eq(tag_id))
                    .to_owned(),
            ),
        )
    }
}
