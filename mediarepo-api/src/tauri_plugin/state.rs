use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use tauri::async_runtime::RwLock;
use tokio::time::Instant;

use crate::client_api::ApiClient;
use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::settings::{load_settings, Repository, Settings};

pub struct ApiState {
    inner: Arc<RwLock<Option<ApiClient>>>,
}

impl ApiState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_api(&self, client: ApiClient) {
        let mut inner = self.inner.write().await;
        let old_client = mem::replace(&mut *inner, Some(client));

        if let Some(client) = old_client {
            let _ = client.exit().await;
        }
    }

    pub async fn api(&self) -> PluginResult<ApiClient> {
        let inner = self.inner.read().await;
        inner
            .clone()
            .ok_or_else(|| PluginError::from("Not connected"))
    }
}

#[derive(Clone)]
pub struct VolatileBuffer {
    pub accessed: bool,
    pub valid_until: Instant,
    pub mime: String,
    pub buf: Vec<u8>,
}

impl VolatileBuffer {
    pub fn new(mime: String, buf: Vec<u8>) -> Self {
        Self {
            accessed: false,
            valid_until: Instant::now() + Duration::from_secs(60),
            mime,
            buf,
        }
    }
}

#[derive(Default)]
pub struct BufferState {
    pub buffer: Arc<Mutex<HashMap<String, VolatileBuffer>>>,
}

impl BufferState {
    /// Checks if an entry for the specific key exists and resets
    /// its state so that it can safely be accessed again.
    pub fn reserve_entry(&self, key: &String) -> bool {
        let mut buffers = self.buffer.lock();
        let entry = buffers.get_mut(key);

        if let Some(entry) = entry {
            entry.accessed = false; // reset that it has been accessed so it can be reused
            true
        } else {
            false
        }
    }

    /// Returns the cloned buffer entry and flags it for expiration
    pub fn get_entry(&self, key: &str) -> Option<VolatileBuffer> {
        let mut buffers = self.buffer.lock();
        let entry = buffers.get_mut(key);

        if let Some(entry) = entry {
            entry.accessed = true;
            entry.valid_until = Instant::now() + Duration::from_secs(10); // time to live is 10 seconds

            Some(entry.clone())
        } else {
            None
        }
    }

    /// Clears all expired entries
    pub fn clear_expired(&self) {
        let mut buffer = self.buffer.lock();
        let keys: Vec<String> = buffer.keys().cloned().collect();

        for key in keys {
            let (accessed, valid_until) = {
                let entry = buffer.get(&key).unwrap();
                (entry.accessed, entry.valid_until.clone())
            };
            if accessed && valid_until < Instant::now() {
                buffer.remove(&key);
            }
        }
    }
}

pub struct AppState {
    pub active_repo: Arc<RwLock<Option<Repository>>>,
    pub settings: Arc<RwLock<Settings>>,
}

impl AppState {
    #[tracing::instrument(level = "debug")]
    pub fn load() -> PluginResult<Self> {
        let settings = load_settings()?;

        let state = Self {
            active_repo: Arc::new(RwLock::new(None)),
            settings: Arc::new(RwLock::new(settings)),
        };

        Ok(state)
    }
}
