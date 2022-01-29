use crate::content_descriptor::ContentDescriptor;
use crate::dao::{DaoContext, DaoProvider};
use crate::file::File;
use crate::file_metadata::FileMetadata;
use crate::namespace::Namespace;
use crate::tag::Tag;
use crate::thumbnail::Thumbnail;
use chrono::{Local, NaiveDateTime};
use mediarepo_core::content_descriptor::{encode_content_descriptor, is_v1_content_descriptor};
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::fs::file_hash_store::FileHashStore;
use mediarepo_core::fs::thumbnail_store::{Dimensions, ThumbnailStore};
use mediarepo_core::itertools::Itertools;
use mediarepo_core::thumbnailer::ThumbnailSize;
use mediarepo_core::utils::parse_namespace_and_tag;
use mediarepo_database::get_database;
use mediarepo_database::queries::analysis::{get_all_counts, Counts};
use sea_orm::DatabaseConnection;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::Cursor;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct Repo {
    db: DatabaseConnection,
    main_storage: FileHashStore,
    thumbnail_storage: ThumbnailStore,
}

impl DaoProvider for Repo {
    fn dao_ctx(&self) -> DaoContext {
        DaoContext {
            db: self.db.clone(),
            main_storage: self.main_storage.clone(),
            thumbnail_storage: self.thumbnail_storage.clone(),
        }
    }
}

impl Repo {
    pub(crate) fn new(
        db: DatabaseConnection,
        file_store_path: PathBuf,
        thumb_store_path: PathBuf,
    ) -> Self {
        Self {
            db,
            main_storage: FileHashStore::new(file_store_path),
            thumbnail_storage: ThumbnailStore::new(thumb_store_path),
        }
    }

    /// Connects to the database with the given uri
    #[tracing::instrument(level = "debug")]
    pub async fn connect<S: AsRef<str> + Debug>(
        uri: S,
        file_store_path: PathBuf,
        thumb_store_path: PathBuf,
    ) -> RepoResult<Self> {
        let db = get_database(uri).await?;
        Ok(Self::new(db, file_store_path, thumb_store_path))
    }

    /// Returns the database of the repo for raw sql queries
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Returns all file metadata entries for the given file ids
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_file_metadata_for_ids(&self, ids: Vec<i64>) -> RepoResult<Vec<FileMetadata>> {
        FileMetadata::all_by_ids(self.db.clone(), ids).await
    }

    /// Adds a file from bytes to the database
    #[tracing::instrument(level = "debug", skip(self, content))]
    pub async fn add_file(
        &self,
        mime_type: Option<String>,
        content: Vec<u8>,
        creation_time: NaiveDateTime,
        change_time: NaiveDateTime,
    ) -> RepoResult<File> {
        let file_size = content.len();
        let reader = Cursor::new(content);
        let cd_binary = self.main_storage.add_file(reader, None).await?;
        let cd = ContentDescriptor::add(self.db.clone(), cd_binary).await?;

        let mime_type = mime_type
            .and_then(|m| mime::Mime::from_str(&m).ok())
            .unwrap_or_else(|| mime::APPLICATION_OCTET_STREAM)
            .to_string();

        let file = File::add(self.db.clone(), cd.id(), mime_type).await?;
        FileMetadata::add(
            self.db.clone(),
            file.id(),
            file_size as i64,
            creation_time,
            change_time,
        )
        .await?;

        Ok(file)
    }

    /// Adds a file to the database by its readable path in the file system
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_file_by_path(&self, path: PathBuf) -> RepoResult<File> {
        let mime_type = mime_guess::from_path(&path).first().map(|m| m.to_string());

        let mut os_file = OpenOptions::new().read(true).open(&path).await?;
        let mut buf = Vec::new();
        os_file.read_to_end(&mut buf).await?;

        self.add_file(
            mime_type,
            buf,
            Local::now().naive_local(),
            Local::now().naive_local(),
        )
        .await
    }

    /// Deletes a file from the database and disk
    #[tracing::instrument(level = "debug", skip(self, file))]
    pub async fn delete_file(&self, file: File) -> RepoResult<()> {
        let cd = file.cd().to_owned();
        let cd_string = file.encoded_cd();
        file.delete().await?;
        self.main_storage.delete_file(&cd).await?;
        self.thumbnail_storage.delete_parent(&cd_string).await?;

        Ok(())
    }

    /// Returns all thumbnails of a file
    pub async fn get_file_thumbnails(&self, file_cd: &[u8]) -> RepoResult<Vec<Thumbnail>> {
        let file_cd = encode_content_descriptor(file_cd);
        let thumbnails = self
            .thumbnail_storage
            .get_thumbnails(&file_cd)
            .await?
            .into_iter()
            .map(|(size, path)| Thumbnail {
                file_hash: file_cd.to_owned(),
                path,
                size,
                mime_type: mime::IMAGE_PNG.to_string(),
            })
            .collect_vec();

        Ok(thumbnails)
    }

    pub async fn get_file_bytes(&self, file: &File) -> RepoResult<Vec<u8>> {
        let mut buf = Vec::new();
        let mut reader = file.get_reader(&self.main_storage).await?;
        reader.read_to_end(&mut buf).await?;

        Ok(buf)
    }

    /// Creates thumbnails of all sizes for a file
    #[tracing::instrument(level = "debug", skip(self, file))]
    pub async fn create_thumbnails_for_file(&self, file: &File) -> RepoResult<Vec<Thumbnail>> {
        let size = ThumbnailSize::Medium;
        let (height, width) = size.dimensions();
        let thumbs = file.create_thumbnail(&self.main_storage, [size]).await?;
        let mut created_thumbs = Vec::with_capacity(1);

        for thumb in thumbs {
            let entry = self
                .store_single_thumbnail(file.encoded_cd(), height, width, thumb)
                .await?;
            created_thumbs.push(entry);
        }

        Ok(created_thumbs)
    }

    #[tracing::instrument(level = "debug", skip(self, file))]
    pub async fn create_file_thumbnail(
        &self,
        file: &File,
        size: ThumbnailSize,
    ) -> RepoResult<Thumbnail> {
        let (height, width) = size.dimensions();
        let thumb = file
            .create_thumbnail(&self.main_storage, [size])
            .await?
            .pop()
            .ok_or_else(|| RepoError::from("Failed to create thumbnail"))?;
        let thumbnail = self
            .store_single_thumbnail(file.encoded_cd(), height, width, thumb)
            .await?;

        Ok(thumbnail)
    }

    /// Stores a single thumbnail
    async fn store_single_thumbnail(
        &self,
        file_hash: String,
        height: u32,
        width: u32,
        thumb: mediarepo_core::thumbnailer::Thumbnail,
    ) -> RepoResult<Thumbnail> {
        let mut buf = Vec::new();
        thumb.write_png(&mut buf)?;
        let size = Dimensions { height, width };
        let path = self
            .thumbnail_storage
            .add_thumbnail(&file_hash, size.clone(), &buf)
            .await?;

        let thumbnail = Thumbnail {
            file_hash,
            path,
            size,
            mime_type: mime::IMAGE_PNG.to_string(),
        };

        Ok(thumbnail)
    }

    /// Returns all tags stored in the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn tags(&self) -> RepoResult<Vec<Tag>> {
        Tag::all(self.db.clone()).await
    }

    /// Returns all namespaces stored in the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn namespaces(&self) -> RepoResult<Vec<Namespace>> {
        Namespace::all(self.db.clone()).await
    }

    /// Converts a list of tag names to tag ids
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn tag_names_to_ids(&self, tags: Vec<String>) -> RepoResult<HashMap<String, i64>> {
        let parsed_tags = tags
            .iter()
            .map(|tag| parse_namespace_and_tag(tag.clone()))
            .unique()
            .collect();

        let db_tags = self.tags_by_names(parsed_tags).await?;
        let tag_map: HashMap<String, i64> =
            HashMap::from_iter(db_tags.into_iter().map(|t| (t.normalized_name(), t.id())));

        Ok(tag_map)
    }

    /// Finds all tags by name
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn tags_by_names(&self, tags: Vec<(Option<String>, String)>) -> RepoResult<Vec<Tag>> {
        Tag::all_by_name(self.db.clone(), tags).await
    }

    /// Finds all tags that are assigned to the given list of hashes
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn find_tags_for_file_identifiers(&self, cds: Vec<Vec<u8>>) -> RepoResult<Vec<Tag>> {
        Tag::for_cd_list(self.db.clone(), cds).await
    }

    /// Adds all tags that are not in the database to the database and returns the ones already existing as well
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn add_all_tags(&self, tags: Vec<(Option<String>, String)>) -> RepoResult<Vec<Tag>> {
        let mut tags_to_add = tags.into_iter().unique().collect_vec();
        let mut namespaces_to_add = tags_to_add
            .iter()
            .filter_map(|(namespace, _)| namespace.clone())
            .unique()
            .collect_vec();

        let mut existing_namespaces =
            Namespace::all_by_name(self.db.clone(), namespaces_to_add.clone()).await?;
        {
            let existing_namespaces_set = existing_namespaces
                .iter()
                .map(|n| n.name().clone())
                .collect::<HashSet<String>>();
            namespaces_to_add.retain(|namespace| !existing_namespaces_set.contains(namespace));
        }
        existing_namespaces
            .append(&mut Namespace::add_all(self.db.clone(), namespaces_to_add).await?);

        let mut existing_tags = self.tags_by_names(tags_to_add.clone()).await?;
        {
            let existing_tags_set = existing_tags
                .iter()
                .map(|t| (t.namespace().map(|n| n.name().clone()), t.name().clone()))
                .collect::<HashSet<(Option<String>, String)>>();

            tags_to_add.retain(|t| !existing_tags_set.contains(t));
        }
        let namespace_map = existing_namespaces
            .into_iter()
            .map(|namespace| (namespace.name().clone(), namespace.id()))
            .collect::<HashMap<String, i64>>();
        let tags_to_add = tags_to_add
            .into_iter()
            .map(|(nsp, name)| (nsp.and_then(|n| namespace_map.get(&n)).map(|i| *i), name))
            .collect_vec();
        existing_tags.append(&mut Tag::add_all(self.db.clone(), tags_to_add).await?);

        Ok(existing_tags)
    }

    /// Adds or finds a tag
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_or_find_tag<S: ToString + Debug>(&self, tag: S) -> RepoResult<Tag> {
        let (namespace, name) = parse_namespace_and_tag(tag.to_string());
        if let Some(namespace) = namespace {
            self.add_or_find_namespaced_tag(name, namespace).await
        } else {
            self.add_or_find_unnamespaced_tag(name).await
        }
    }

    /// Adds or finds an unnamespaced tag
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_or_find_unnamespaced_tag(&self, name: String) -> RepoResult<Tag> {
        if let Some(tag) = Tag::by_name(self.db.clone(), &name, None).await? {
            Ok(tag)
        } else {
            self.add_unnamespaced_tag(name).await
        }
    }

    /// Adds an unnamespaced tag
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_unnamespaced_tag(&self, name: String) -> RepoResult<Tag> {
        Tag::add(self.db.clone(), name, None).await
    }

    /// Adds or finds a namespaced tag
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_or_find_namespaced_tag(
        &self,
        name: String,
        namespace: String,
    ) -> RepoResult<Tag> {
        if let Some(tag) = Tag::by_name(self.db.clone(), &name, Some(namespace.clone())).await? {
            Ok(tag)
        } else {
            self.add_namespaced_tag(name, namespace).await
        }
    }

    /// Adds a namespaced tag
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_namespaced_tag(&self, name: String, namespace: String) -> RepoResult<Tag> {
        let namespace =
            if let Some(namespace) = Namespace::by_name(self.db.clone(), &namespace).await? {
                namespace
            } else {
                Namespace::add(self.db.clone(), namespace).await?
            };
        Tag::add(self.db.clone(), name, Some(namespace.id())).await
    }

    /// Returns the size of the main storage
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_main_store_size(&self) -> RepoResult<u64> {
        self.main_storage.get_size().await
    }

    /// Returns the size of the thumbnail storage
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_thumb_store_size(&self) -> RepoResult<u64> {
        self.thumbnail_storage.get_size().await
    }

    /// Returns all entity counts
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_counts(&self) -> RepoResult<Counts> {
        get_all_counts(&self.db).await
    }

    pub async fn migrate(&self) -> RepoResult<()> {
        let cds = ContentDescriptor::all(self.db.clone()).await?;

        tracing::info!("Converting content descriptors to v2 format...");
        let mut converted_count = 0;

        for mut cd in cds {
            if is_v1_content_descriptor(cd.descriptor()) {
                let src_cd = cd.descriptor().to_owned();
                cd.convert_v1_to_v2().await?;
                let dst_cd = cd.descriptor().to_owned();
                self.main_storage.rename_file(&src_cd, &dst_cd).await?;
                self.thumbnail_storage
                    .rename_parent(
                        encode_content_descriptor(&src_cd),
                        encode_content_descriptor(&dst_cd),
                    )
                    .await?;
                converted_count += 1;
            }
        }
        tracing::info!("Converted {} descriptors", converted_count);

        Ok(())
    }
}
