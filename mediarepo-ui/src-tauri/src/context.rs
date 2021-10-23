use crate::commands::repo::Repository;
use crate::settings::Settings;
use rmp_ipc::ipc::context::Context as IPCContext;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tokio::sync::RwLock;

pub struct OnceBuffer {
  pub mime: String,
  pub buf: Vec<u8>,
}

#[derive(Clone)]
pub struct Context {
  pub active_repository: Arc<RwLock<Option<Repository>>>,
  pub ipc: Arc<RwLock<Option<IPCContext>>>,
  pub settings: Arc<RwLock<Settings>>,
  pub once_buffers: Arc<StdMutex<HashMap<String, OnceBuffer>>>,
}

impl Context {
  pub fn new(settings: Settings) -> Self {
    Self {
      ipc: Arc::new(RwLock::new(None)),
      active_repository: Arc::new(RwLock::new(None)),
      settings: Arc::new(RwLock::new(settings)),
      once_buffers: Arc::new(StdMutex::new(HashMap::new())),
    }
  }
}
