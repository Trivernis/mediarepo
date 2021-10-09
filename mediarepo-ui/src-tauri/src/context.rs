use std::sync::Arc;
use rmp_ipc::client::IPCClient;
use tokio::sync::RwLock;
use crate::commands::repo::Repository;
use crate::settings::Settings;

#[derive(Clone)]
pub struct Context {
  pub active_repository: Option<Repository>,
  pub client: Option<Arc<IPCClient>>,
  pub settings: Arc<RwLock<Settings>>
}

impl Context {
  pub fn new(settings: Settings) -> Self {
    Self {
      client: None,
      active_repository: None,
      settings: Arc::new(RwLock::new(settings))
    }
  }
}
