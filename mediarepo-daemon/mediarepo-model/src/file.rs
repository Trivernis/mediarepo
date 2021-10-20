use crate::file_type::FileType;
use crate::storage::Storage;
use crate::tag::Tag;
use crate::thumbnail::Thumbnail;
use chrono::{Local, NaiveDateTime};
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::image::GenericImageView;
use mediarepo_core::image_processing::{
    create_thumbnail, get_image_bytes_png, read_image, ThumbnailSize,
};
use mediarepo_database::entities::file;
use mediarepo_database::entities::hash;
use mediarepo_database::entities::hash_tag;
use mediarepo_database::entities::namespace;
use mediarepo_database::entities::tag;
use mime::Mime;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{JoinType, QuerySelect};
use tokio::io::BufReader;

#[derive(Clone)]
pub struct File {
    db: DatabaseConnection,
    model: file::Model,
    hash: hash::Model,
}

impl File {
    pub(crate) fn new(db: DatabaseConnection, model: file::Model, hash: hash::Model) -> Self {
        Self { db, model, hash }
    }

    /// Returns a list of all known stored files
    pub async fn all(db: DatabaseConnection) -> RepoResult<Vec<File>> {
        let files: Vec<(file::Model, Option<hash::Model>)> = file::Entity::find()
            .find_also_related(hash::Entity)
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
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        if let Some((model, Some(hash))) = file::Entity::find_by_id(id)
            .find_also_related(hash::Entity)
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
    pub async fn by_hash<S: AsRef<str>>(
        db: DatabaseConnection,
        hash: S,
    ) -> RepoResult<Option<Self>> {
        if let Some((hash, Some(model))) = hash::Entity::find()
            .filter(hash::Column::Value.eq(hash.as_ref()))
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

    /// Adds a file with its hash to the database
    pub(crate) async fn add(
        db: DatabaseConnection,
        storage_id: i64,
        hash_id: i64,
        file_type: FileType,
        mime_type: Option<String>,
    ) -> RepoResult<Self> {
        let now = Local::now().naive_local();
        let file = file::ActiveModel {
            hash_id: Set(hash_id),
            file_type: Set(file_type as u32),
            mime_type: Set(mime_type),
            storage_id: Set(storage_id),
            import_time: Set(now.clone()),
            creation_time: Set(now.clone()),
            change_time: Set(now),
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
    pub fn hash(&self) -> &String {
        &self.hash.value
    }

    /// Returns the type of the file
    pub fn file_type(&self) -> FileType {
        match self.model.file_type {
            1 => FileType::Image,
            2 => FileType::Video,
            3 => FileType::Audio,
            _ => FileType::Unknown,
        }
    }

    /// Returns the optional mime type of the file
    pub fn mime_type(&self) -> &Option<String> {
        &self.model.mime_type
    }

    /// Returns the optional name of the file
    pub fn name(&self) -> &Option<String> {
        &self.model.name
    }

    /// Returns the comment of the file
    pub fn comment(&self) -> &Option<String> {
        &self.model.comment
    }

    /// Returns the import time of the file
    pub fn import_time(&self) -> &NaiveDateTime {
        &self.model.import_time
    }

    /// Returns the datetime when the file was created
    pub fn creation_time(&self) -> &NaiveDateTime {
        &self.model.creation_time
    }

    /// Returns the last time the file was changed
    pub fn change_time(&self) -> &NaiveDateTime {
        &self.model.change_time
    }

    /// Returns the storage where the file is stored
    pub async fn storage(&self) -> RepoResult<Storage> {
        let storage = Storage::by_id(self.db.clone(), self.model.storage_id)
            .await?
            .expect("The FK storage_id doesn't exist?!");

        Ok(storage)
    }

    /// Returns a list of thumbnails for the file
    pub async fn thumbnails(&self) -> RepoResult<Vec<Thumbnail>> {
        Thumbnail::for_file_id(self.db.clone(), self.model.id).await
    }

    /// Returns the list of tags of the file
    pub async fn tags(&self) -> RepoResult<Vec<Tag>> {
        let tags: Vec<(tag::Model, Option<namespace::Model>)> = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .join(JoinType::LeftJoin, hash_tag::Relation::Tag.def().rev())
            .join(JoinType::InnerJoin, hash_tag::Relation::Hash.def())
            .filter(hash::Column::Id.eq(self.hash.id))
            .all(&self.db)
            .await?;
        let tags = tags
            .into_iter()
            .map(|(tag, namespace)| Tag::new(self.db.clone(), tag, namespace))
            .collect();

        Ok(tags)
    }

    /// Changes the name of the file
    pub async fn set_name<S: ToString>(&mut self, name: S) -> RepoResult<()> {
        let mut active_file = self.get_active_model();
        active_file.name = Set(Some(name.to_string()));
        let active_file = active_file.update(&self.db).await?;
        self.model.name = active_file.name.unwrap();

        Ok(())
    }

    /// Changes the comment of the file
    pub async fn set_comment<S: ToString>(&mut self, comment: S) -> RepoResult<()> {
        let mut active_file = self.get_active_model();
        active_file.comment = Set(Some(comment.to_string()));
        let active_file = active_file.update(&self.db).await?;
        self.model.comment = active_file.comment.unwrap();

        Ok(())
    }

    /// Changes the type of the file
    pub async fn set_file_type(&mut self, file_type: FileType) -> RepoResult<()> {
        let mut active_file = self.get_active_model();
        active_file.file_type = Set(file_type as u32);
        let active_file = active_file.update(&self.db).await?;
        self.model.file_type = active_file.file_type.unwrap();

        Ok(())
    }

    /// Adds a single tag to the file
    pub async fn add_tag(&mut self, tag_id: i64) -> RepoResult<()> {
        let hash_id = self.hash.id;
        let active_model = hash_tag::ActiveModel {
            hash_id: Set(hash_id),
            tag_id: Set(tag_id),
        };
        active_model.insert(&self.db).await?;
        Ok(())
    }

    /// Adds multiple tags to the file at once
    pub async fn add_tags(&self, tag_ids: Vec<i64>) -> RepoResult<()> {
        let hash_id = self.hash.id;
        let models: Vec<hash_tag::ActiveModel> = tag_ids
            .into_iter()
            .map(|tag_id| hash_tag::ActiveModel {
                hash_id: Set(hash_id),
                tag_id: Set(tag_id),
            })
            .collect();
        hash_tag::Entity::insert_many(models).exec(&self.db).await?;

        Ok(())
    }

    /// Returns the reader for the file
    pub async fn get_reader(&self) -> RepoResult<BufReader<tokio::fs::File>> {
        let storage = self.storage().await?;

        storage.get_file_reader(&self.hash.value).await
    }

    /// Creates a thumbnail for the file
    pub async fn create_thumbnail(
        &self,
        size: ThumbnailSize,
    ) -> RepoResult<(Vec<u8>, Mime, (u32, u32))> {
        match self.file_type() {
            FileType::Image => self.create_image_thumbnail(size).await,
            _ => Err(RepoError::from(
                "Unsupported file type for thumbnail generation",
            )),
        }
    }

    /// Creates a thumbnail for an image
    async fn create_image_thumbnail(
        &self,
        size: ThumbnailSize,
    ) -> RepoResult<(Vec<u8>, Mime, (u32, u32))> {
        let mut reader = self.get_reader().await?;
        let image = read_image(&mut reader).await?;
        let thumb_image = create_thumbnail(image, size);
        let actual_size = (thumb_image.height(), thumb_image.width());
        let bytes = get_image_bytes_png(thumb_image)?;

        Ok((bytes, mime::IMAGE_PNG, actual_size))
    }

    /// Returns the active model of the file with only the id set
    fn get_active_model(&self) -> file::ActiveModel {
        file::ActiveModel {
            id: Set(self.id()),
            ..Default::default()
        }
    }
}
