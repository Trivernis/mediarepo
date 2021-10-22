use crate::commands::repo::Repository;
use crate::settings::Settings;
use rmp_ipc::ipc::context::Context as IPCContext;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Context {
  pub active_repository: Arc<RwLock<Option<Repository>>>,
  pub ipc: Arc<RwLock<Option<IPCContext>>>,
  pub settings: Arc<RwLock<Settings>>,
}

impl Context {
  pub fn new(settings: Settings) -> Self {
    Self {
      ipc: Arc::new(RwLock::new(None)),
      active_repository: Arc::new(RwLock::new(None)),
      settings: Arc::new(RwLock::new(settings)),
    }
  }
}
