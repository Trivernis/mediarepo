use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use parking_lot::RwLock as ParkingRwLock;
use tauri::async_runtime::RwLock;
use tokio::time::Instant;

use crate::client_api::ApiClient;
use crate::daemon_management::cli::DaemonCli;
use crate::daemon_management::find_daemon_executable;
use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::settings::{load_settings, save_settings, Repository, Settings};

pub struct ApiState {
    inner: Arc<RwLock<Option<ApiClient>>>,
}

unsafe impl Send for ApiState {}
unsafe impl Sync for ApiState {}

impl ApiState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(None)),
        }
    }

    /// Sets the active api client and disconnects the old one
    pub async fn set_api(&self, client: ApiClient) {
        let mut inner = self.inner.write().await;
        let old_client = mem::replace(&mut *inner, Some(client));

        if let Some(client) = old_client {
            let _ = client.exit().await;
        }
    }

    /// Disconnects the api client
    pub async fn disconnect(&self) {
        let mut inner = self.inner.write().await;
        let old_client = mem::take(&mut *inner);

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
    pub valid_until: Instant,
    pub mime: String,
    pub buf: Vec<u8>,
}

impl VolatileBuffer {
    pub fn new(mime: String, buf: Vec<u8>) -> Self {
        Self {
            valid_until: Instant::now() + Duration::from_secs(120), // buffers that weren't accessed get deleted after 2 minutes
            mime,
            buf,
        }
    }
}

#[derive(Default, Clone)]
pub struct BufferState {
    pub buffer: Arc<ParkingRwLock<HashMap<String, Mutex<VolatileBuffer>>>>,
}

impl BufferState {
    /// Adds a cached buffer to the buffer state
    pub fn add_entry(&self, key: String, mime: String, bytes: Vec<u8>) {
        let mut buffers = self.buffer.write();
        let buffer = VolatileBuffer::new(mime, bytes);
        buffers.insert(key, Mutex::new(buffer));
    }

    /// Returns the cloned buffer entry and flags it for expiration
    pub fn get_entry(&self, key: &str) -> Option<VolatileBuffer> {
        let buffers = self.buffer.read();
        let entry = buffers.get(key);

        if let Some(entry) = entry {
            let mut entry = entry.lock();
            entry.valid_until = Instant::now() + Duration::from_secs(30); // ttl is 30 seconds after being accessed

            Some(entry.clone())
        } else {
            None
        }
    }

    /// Clears all expired entries
    pub fn clear_expired(&self) {
        let keys: Vec<String> = {
            let buffer = self.buffer.read();
            buffer.keys().cloned().collect()
        };

        for key in keys {
            let valid_until = {
                let buffer = self.buffer.read();
                let entry = buffer.get(&key).unwrap().lock();
                entry.valid_until.clone()
            };
            if valid_until < Instant::now() {
                let mut buffer = self.buffer.write();
                buffer.remove(&key);
            }
        }
    }

    /// Clears the buffer completely
    pub fn clear(&self) {
        let mut buffer = self.buffer.write();
        buffer.clear();
    }

    /// Trims the buffer to the given target size
    pub fn trim_to_size(&self, target_size: usize) {
        let mut size = self.get_size();
        if size < target_size {
            return;
        }

        let mut keys: Vec<String> = {
            let buffer = self.buffer.read();
            buffer.keys().cloned().collect()
        };
        keys.reverse();

        while size > target_size && keys.len() > 0 {
            let key = keys.pop().unwrap();
            let mut buffers = self.buffer.write();

            if let Some(entry) = buffers.remove(&key) {
                size -= entry.lock().buf.len();
            }
        }
    }

    /// Calculates the size of the whole buffer
    pub fn get_size(&self) -> usize {
        let buffer = self.buffer.read();
        let mut size = 0;

        for value in buffer.deref().values() {
            let value = value.lock();
            size += value.buf.len();
        }

        size
    }
}

pub struct AppState {
    pub active_repo: Arc<RwLock<Option<Repository>>>,
    pub settings: Arc<RwLock<Settings>>,
    pub running_daemons: Arc<RwLock<HashMap<String, DaemonCli>>>,
}

impl AppState {
    #[tracing::instrument(level = "debug")]
    pub fn load() -> PluginResult<Self> {
        let settings = load_settings()?;

        let state = Self {
            active_repo: Default::default(),
            settings: Arc::new(RwLock::new(settings)),
            running_daemons: Default::default(),
        };

        Ok(state)
    }

    /// Returns the daemon cli client
    pub async fn get_daemon_cli(&self, repo_path: String) -> PluginResult<DaemonCli> {
        let mut settings = self.settings.write().await;
        if settings.daemon_path.is_none() {
            settings.daemon_path =
                find_daemon_executable().map(|p| p.to_string_lossy().to_string());
            save_settings(&settings)?;
        }
        let path = settings
            .daemon_path
            .clone()
            .ok_or_else(|| PluginError::from("Missing daemon executable"))?;
        let cli = DaemonCli::new(path, repo_path);

        Ok(cli)
    }

    /// Adds a started daemon to the running daemons
    pub async fn add_started_daemon(&self, daemon: DaemonCli) {
        let mut daemons = self.running_daemons.write().await;
        daemons.insert(daemon.repo_path().to_owned(), daemon);
    }

    /// Tries to stop a running daemon
    pub async fn stop_running_daemon(&self, repo_path: &String) -> PluginResult<()> {
        let mut daemons = self.running_daemons.write().await;

        if let Some(mut daemon) = daemons.remove(repo_path) {
            daemon.stop_daemon().await?;
        }

        Ok(())
    }
}
