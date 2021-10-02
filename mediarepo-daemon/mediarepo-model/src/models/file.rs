use mediarepo_database::entities::file;
use mediarepo_database::entities::file::Model as FileModel;
use sea_orm::DatabaseConnection;
use tokio::fs;

pub struct File {
    db: DatabaseConnection,
    model: FileModel,
}
