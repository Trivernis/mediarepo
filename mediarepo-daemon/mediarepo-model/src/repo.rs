use crate::file::File;
use crate::file_metadata::FileMetadata;
use crate::namespace::Namespace;
use crate::storage::Storage;
use crate::tag::Tag;
use crate::thumbnail::Thumbnail;
use chrono::{Local, NaiveDateTime};
use mediarepo_core::content_descriptor::encode_content_descriptor;
use mediarepo_core::error::{RepoError, RepoResult};
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
    main_storage: Option<Storage>,
    thumbnail_storage: Option<ThumbnailStore>,
}

impl Repo {
    pub(crate) fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            main_storage: None,
            thumbnail_storage: None,
        }
    }

    /// Connects to the database with the given uri
    #[tracing::instrument(level = "debug")]
    pub async fn connect<S: AsRef<str> + Debug>(uri: S) -> RepoResult<Self> {
        let db = get_database(uri).await?;
        Ok(Self::new(db))
    }

    /// Returns the database of the repo for raw sql queries
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Returns all available storages
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn storages(&self) -> RepoResult<Vec<Storage>> {
        Storage::all(self.db.clone()).await
    }

    /// Returns a storage by path
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn storage_by_path<S: ToString + Debug>(
        &self,
        path: S,
    ) -> RepoResult<Option<Storage>> {
        Storage::by_path(self.db.clone(), path).await
    }

    /// Sets the main storage
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_main_storage<S: ToString + Debug>(&mut self, name: S) -> RepoResult<()> {
        self.main_storage = Storage::by_name(self.db.clone(), name.to_string()).await?;
        Ok(())
    }

    /// Sets the default thumbnail storage
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_thumbnail_storage(&mut self, path: PathBuf) -> RepoResult<()> {
        self.thumbnail_storage = Some(ThumbnailStore::new(path));
        Ok(())
    }

    /// Adds a storage to the repository
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_storage<S1: ToString + Debug, S2: ToString + Debug>(
        &self,
        name: S1,
        path: S2,
    ) -> RepoResult<Storage> {
        Storage::create(self.db.clone(), name, path).await
    }

    /// Returns a file by its mapped hash
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn file_by_cd<S: AsRef<str> + Debug>(&self, hash: S) -> RepoResult<Option<File>> {
        File::by_cd(self.db.clone(), hash).await
    }

    /// Returns a file by id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn file_by_id(&self, id: i64) -> RepoResult<Option<File>> {
        File::by_id(self.db.clone(), id).await
    }

    /// Returns a list of all stored files
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn files(&self) -> RepoResult<Vec<File>> {
        File::all(self.db.clone()).await
    }

    /// Finds all files by a list of tags
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn find_files_by_tags(
        &self,
        tags: Vec<Vec<(String, bool)>>,
    ) -> RepoResult<Vec<File>> {
        let parsed_tags = tags
            .iter()
            .flat_map(|e| e.into_iter().map(|t| parse_namespace_and_tag(t.0.clone())))
            .unique()
            .collect();

        let db_tags = self.tags_by_names(parsed_tags).await?;
        let tag_map: HashMap<String, i64> =
            HashMap::from_iter(db_tags.into_iter().map(|t| (t.normalized_name(), t.id())));

        let tag_ids = process_filters_with_tag_ids(tags, tag_map);

        File::find_by_tags(self.db.clone(), tag_ids).await
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
        let storage = self.get_main_storage()?;
        let file_size = content.len();
        let reader = Cursor::new(content);
        let hash = storage.store_entry(reader).await?;

        let mime_type = mime_type
            .and_then(|m| mime::Mime::from_str(&m).ok())
            .unwrap_or_else(|| mime::APPLICATION_OCTET_STREAM)
            .to_string();

        let file = File::add(self.db.clone(), storage.id(), hash.id(), mime_type).await?;
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

    /// Returns all thumbnails of a file
    pub async fn get_file_thumbnails(&self, file_cd: &[u8]) -> RepoResult<Vec<Thumbnail>> {
        let thumb_store = self.get_thumbnail_storage()?;
        let file_cd = encode_content_descriptor(file_cd);
        let thumbnails = thumb_store
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

    /// Creates thumbnails of all sizes for a file
    #[tracing::instrument(level = "debug", skip(self, file))]
    pub async fn create_thumbnails_for_file(&self, file: &File) -> RepoResult<Vec<Thumbnail>> {
        let thumb_storage = self.get_thumbnail_storage()?;
        let size = ThumbnailSize::Medium;
        let (height, width) = size.dimensions();
        let thumbs = file.create_thumbnail([size]).await?;
        let mut created_thumbs = Vec::with_capacity(1);

        for thumb in thumbs {
            let entry = self
                .store_single_thumbnail(file.encoded_cd(), thumb_storage, height, width, thumb)
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
        let thumb_storage = self.get_thumbnail_storage()?;
        let (height, width) = size.dimensions();
        let thumb = file
            .create_thumbnail([size])
            .await?
            .pop()
            .ok_or_else(|| RepoError::from("Failed to create thumbnail"))?;
        let thumbnail = self
            .store_single_thumbnail(file.encoded_cd(), thumb_storage, height, width, thumb)
            .await?;

        Ok(thumbnail)
    }

    /// Stores a single thumbnail
    async fn store_single_thumbnail(
        &self,
        file_hash: String,
        thumb_storage: &ThumbnailStore,
        height: u32,
        width: u32,
        thumb: mediarepo_core::thumbnailer::Thumbnail,
    ) -> RepoResult<Thumbnail> {
        let mut buf = Vec::new();
        thumb.write_png(&mut buf)?;
        let size = Dimensions { height, width };
        let path = thumb_storage
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

    /// Finds all tags by name
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn tags_by_names(&self, tags: Vec<(Option<String>, String)>) -> RepoResult<Vec<Tag>> {
        Tag::all_by_name(self.db.clone(), tags).await
    }

    /// Finds all tags that are assigned to the given list of hashes
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn find_tags_for_file_identifiers(
        &self,
        hashes: Vec<Vec<u8>>,
    ) -> RepoResult<Vec<Tag>> {
        Tag::for_hash_list(self.db.clone(), hashes).await
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
        let main_storage = self.get_main_storage()?;
        main_storage.get_size().await
    }

    /// Returns the size of the thumbnail storage
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_thumb_store_size(&self) -> RepoResult<u64> {
        let thumb_storage = self.get_thumbnail_storage()?;
        thumb_storage.get_size().await
    }

    /// Returns all entity counts
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_counts(&self) -> RepoResult<Counts> {
        get_all_counts(&self.db).await
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_main_storage(&self) -> RepoResult<&Storage> {
        if let Some(storage) = &self.main_storage {
            Ok(storage)
        } else {
            Err(RepoError::from("No main storage configured."))
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_thumbnail_storage(&self) -> RepoResult<&ThumbnailStore> {
        if let Some(storage) = &self.thumbnail_storage {
            Ok(storage)
        } else {
            Err(RepoError::from("No thumbnail storage configured."))
        }
    }
}

fn process_filters_with_tag_ids(
    filters: Vec<Vec<(String, bool)>>,
    tag_ids: HashMap<String, i64>,
) -> Vec<Vec<(i64, bool)>> {
    let mut id_filters = Vec::new();

    for expression in filters {
        let mut id_sub_filters = Vec::new();
        let mut negated_wildcard_filters = Vec::new();

        for (tag, negate) in expression {
            if tag.ends_with("*") {
                let tag_prefix = tag.trim_end_matches('*');
                let mut found_tag_ids = tag_ids
                    .iter()
                    .filter(|(k, _)| k.starts_with(tag_prefix))
                    .map(|(_, id)| (*id, negate))
                    .collect::<Vec<(i64, bool)>>();

                if negate {
                    negated_wildcard_filters.push(found_tag_ids)
                } else {
                    id_sub_filters.append(&mut found_tag_ids);
                }
            } else {
                if let Some(id) = tag_ids.get(&tag) {
                    id_sub_filters.push((*id, negate));
                }
            }
        }
        if !negated_wildcard_filters.is_empty() {
            for wildcard_filter in negated_wildcard_filters {
                for query in wildcard_filter {
                    let mut sub_filters = id_sub_filters.clone();
                    sub_filters.push(query);
                    id_filters.push(sub_filters)
                }
            }
        } else if !id_sub_filters.is_empty() {
            id_filters.push(id_sub_filters);
        }
    }

    id_filters
}
