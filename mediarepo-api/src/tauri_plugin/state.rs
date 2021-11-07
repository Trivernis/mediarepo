use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use parking_lot::RwLock as ParkingRwLock;
use tauri::async_runtime::RwLock;
use tokio::time::Instant;

use crate::client_api::protocol::ApiProtocolListener;
use crate::client_api::ApiClient;
use crate::daemon_management::cli::DaemonCli;
use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::settings::{load_settings, Repository, Settings};

pub struct ApiState {
    inner: Arc<RwLock<Option<ApiClient<ApiProtocolListener>>>>,
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
    pub async fn set_api(&self, client: ApiClient<ApiProtocolListener>) {
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

    pub async fn api(&self) -> PluginResult<ApiClient<ApiProtocolListener>> {
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
    /// Checks if an entry for the specific key exists and resets
    /// its state so that it can safely be accessed again.
    pub fn reserve_entry(&self, key: &String) -> bool {
        let buffers = self.buffer.read();
        let entry = buffers.get(key);

        if let Some(entry) = entry {
            let mut entry = entry.lock();
            entry.valid_until = Instant::now() + Duration::from_secs(120); // reset the timer so that it can be accessed again
            true
        } else {
            false
        }
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
    pub async fn get_daemon_cli(&self, repo_path: String) -> DaemonCli {
        let settings = self.settings.read().await;
        let path = settings.daemon_path.clone();

        DaemonCli::new(path, repo_path)
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
