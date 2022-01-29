use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::settings::Settings;

#[derive(Clone, Default)]
pub struct Context {
    pub settings: Arc<Mutex<Settings>>,
    pub database: DatabaseConnection,
}
