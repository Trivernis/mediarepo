use crate::storage::Storage;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::hash;
use mediarepo_database::entities::thumbnail;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;

pub struct Thumbnail {
    db: DatabaseConnection,
    model: thumbnail::Model,
    hash: hash::Model,
}

impl Thumbnail {
    pub(crate) fn new(db: DatabaseConnection, model: thumbnail::Model, hash: hash::Model) -> Self {
        Self { db, model, hash }
    }

    /// Returns the thumbnail by id
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let model: Option<(thumbnail::Model, Option<hash::Model>)> =
            thumbnail::Entity::find_by_id(id)
                .find_also_related(hash::Entity)
                .one(&db)
                .await?;

        if let Some((model, Some(hash))) = model {
            Ok(Some(Self::new(db, model, hash)))
        } else {
            Ok(None)
        }
    }

    /// Returns all thumbnails for a given file
    pub async fn for_file_id(db: DatabaseConnection, file_id: i64) -> RepoResult<Vec<Self>> {
        let thumb_models: Vec<(thumbnail::Model, Option<hash::Model>)> = thumbnail::Entity::find()
            .filter(thumbnail::Column::FileId.eq(file_id))
            .find_also_related(hash::Entity)
            .all(&db)
            .await?;

        Ok(thumb_models
            .into_iter()
            .filter_map(|(m, h)| Some(Self::new(db.clone(), m, h?)))
            .collect())
    }

    pub fn id(&self) -> i64 {
        self.model.id
    }

    pub fn hash(&self) -> &String {
        &self.hash.value
    }

    pub fn height(&self) -> i32 {
        self.model.height
    }

    pub fn width(&self) -> i32 {
        self.model.width
    }

    /// Returns the storage for the thumbnail
    pub async fn storage(&self) -> RepoResult<Storage> {
        let storage = Storage::by_id(self.db.clone(), self.model.storage_id)
            .await?
            .expect("The FK storage_id doesn't exist?!");

        Ok(storage)
    }
}
