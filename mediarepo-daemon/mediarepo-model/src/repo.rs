use crate::file::File;
use crate::file_type::FileType;
use crate::namespace::Namespace;
use crate::storage::Storage;
use crate::tag::Tag;
use crate::thumbnail::Thumbnail;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::image_processing::ThumbnailSize;
use mediarepo_core::utils::parse_namespace_and_tag;
use mediarepo_database::get_database;
use sea_orm::DatabaseConnection;
use std::io::Cursor;
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::BufReader;

#[derive(Clone)]
pub struct Repo {
    db: DatabaseConnection,
    main_storage: Option<Storage>,
    thumbnail_storage: Option<Storage>,
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
    pub async fn connect<S: AsRef<str>>(uri: S) -> RepoResult<Self> {
        let db = get_database(uri).await?;
        Ok(Self::new(db))
    }

    /// Returns all available storages
    pub async fn storages(&self) -> RepoResult<Vec<Storage>> {
        Storage::all(self.db.clone()).await
    }

    /// Returns a storage by path
    pub async fn storage_by_path<S: ToString>(&self, path: S) -> RepoResult<Option<Storage>> {
        Storage::by_path(self.db.clone(), path).await
    }

    /// Sets the main storage
    pub async fn set_main_storage<S: ToString>(&mut self, path: S) -> RepoResult<()> {
        self.main_storage = Storage::by_path(self.db.clone(), path).await?;
        Ok(())
    }

    /// Sets the default thumbnail storage
    pub async fn set_thumbnail_storage<S: ToString>(&mut self, path: S) -> RepoResult<()> {
        self.thumbnail_storage = Storage::by_path(self.db.clone(), path).await?;
        Ok(())
    }

    /// Adds a storage to the repository
    pub async fn add_storage<S1: ToString, S2: ToString>(
        &self,
        name: S1,
        path: S2,
    ) -> RepoResult<Storage> {
        Storage::create(self.db.clone(), name, path).await
    }

    /// Returns a file by its mapped hash
    pub async fn file_by_hash<S: AsRef<str>>(&self, hash: S) -> RepoResult<Option<File>> {
        File::by_hash(self.db.clone(), hash).await
    }

    /// Returns a file by id
    pub async fn file_by_id(&self, id: i64) -> RepoResult<Option<File>> {
        File::by_id(self.db.clone(), id).await
    }

    /// Returns a list of all stored files
    pub async fn files(&self) -> RepoResult<Vec<File>> {
        File::all(self.db.clone()).await
    }

    /// Adds a file to the database by its readable path in the file system
    pub async fn add_file_by_path(&self, path: PathBuf) -> RepoResult<File> {
        let mime_match = mime_guess::from_path(&path).first();

        let (mime_type, file_type) = if let Some(mime) = mime_match {
            (Some(mime.clone().to_string()), FileType::from(mime))
        } else {
            (None, FileType::Unknown)
        };
        let os_file = OpenOptions::new().read(true).open(&path).await?;
        let reader = BufReader::new(os_file);

        let storage = self.get_main_storage()?;
        let hash = storage.store_entry(reader).await?;
        File::add(
            self.db.clone(),
            storage.id(),
            hash.id(),
            file_type,
            mime_type,
        )
        .await
    }

    /// Returns a thumbnail by its hash
    pub async fn thumbnail_by_hash<S: AsRef<str>>(&self, hash: S) -> RepoResult<Option<Thumbnail>> {
        Thumbnail::by_hash(self.db.clone(), hash).await
    }

    /// Creates thumbnails of all sizes for a file
    pub async fn create_thumbnails_for_file(&self, file: File) -> RepoResult<()> {
        let thumb_storage = self.get_thumbnail_storage()?;
        for size in [
            ThumbnailSize::Small,
            ThumbnailSize::Medium,
            ThumbnailSize::Large,
        ] {
            let (bytes, mime, (height, width)) = file.create_thumbnail(size).await?;
            let hash = thumb_storage.store_entry(Cursor::new(bytes)).await?;
            Thumbnail::add(
                self.db.clone(),
                hash.id(),
                file.id(),
                thumb_storage.id(),
                height as i32,
                width as i32,
                Some(mime.to_string()),
            )
            .await?;
        }

        Ok(())
    }

    /// Returns all tags stored in the database
    pub async fn tags(&self) -> RepoResult<Vec<Tag>> {
        Tag::all(self.db.clone()).await
    }

    /// Adds or finds a tag
    pub async fn add_or_find_tag<S: ToString>(&self, tag: S) -> RepoResult<Tag> {
        let (namespace, name) = parse_namespace_and_tag(tag.to_string());
        if let Some(namespace) = namespace {
            self.add_or_find_namespaced_tag(name, namespace).await
        } else {
            self.add_or_find_unnamespaced_tag(name).await
        }
    }

    /// Adds or finds an unnamespaced tag
    async fn add_or_find_unnamespaced_tag(&self, name: String) -> RepoResult<Tag> {
        if let Some(tag) = Tag::by_name(self.db.clone(), &name).await? {
            Ok(tag)
        } else {
            self.add_unnamespaced_tag(name).await
        }
    }

    /// Adds an unnamespaced tag
    async fn add_unnamespaced_tag(&self, name: String) -> RepoResult<Tag> {
        Tag::add(self.db.clone(), name, None).await
    }

    /// Adds or finds a namespaced tag
    async fn add_or_find_namespaced_tag(&self, name: String, namespace: String) -> RepoResult<Tag> {
        if let Some(tag) = Tag::by_name_and_namespace(self.db.clone(), &name, &namespace).await? {
            Ok(tag)
        } else {
            self.add_namespaced_tag(name, namespace).await
        }
    }

    /// Adds a namespaced tag
    async fn add_namespaced_tag(&self, name: String, namespace: String) -> RepoResult<Tag> {
        let namespace =
            if let Some(namespace) = Namespace::by_name(self.db.clone(), &namespace).await? {
                namespace
            } else {
                Namespace::add(self.db.clone(), namespace).await?
            };
        Tag::add(self.db.clone(), name, Some(namespace.id())).await
    }

    fn get_main_storage(&self) -> RepoResult<&Storage> {
        if let Some(storage) = &self.main_storage {
            Ok(storage)
        } else {
            Err(RepoError::from("No main storage configured."))
        }
    }

    fn get_thumbnail_storage(&self) -> RepoResult<&Storage> {
        if let Some(storage) = &self.thumbnail_storage {
            Ok(storage)
        } else {
            Err(RepoError::from("No thumbnail storage configured."))
        }
    }
}
