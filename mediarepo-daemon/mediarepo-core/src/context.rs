use crate::settings::Settings;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct Context {
    pub settings: Arc<Mutex<Settings>>,
    pub database: DatabaseConnection,
}
