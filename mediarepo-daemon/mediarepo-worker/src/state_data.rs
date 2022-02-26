use mediarepo_core::bincode;
use mediarepo_core::error::RepoResult;
use mediarepo_logic::dao::job::JobDao;
use mediarepo_logic::dto::UpsertJobStateDto;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct StateData {
    job_id: i64,
    inner: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    changed_keys: Arc<RwLock<HashSet<String>>>,
}

impl StateData {
    /// Loads the state from the database
    pub async fn load(job_dao: JobDao, job_id: i64) -> RepoResult<Self> {
        let states = job_dao.states_for_job_id(job_id).await?;
        let states_map = states
            .into_iter()
            .map(|s| (s.key().to_owned(), s.into_value()))
            .collect::<HashMap<String, Vec<u8>>>();

        Ok(Self {
            job_id,
            inner: Arc::new(RwLock::new(states_map)),
            changed_keys: Default::default(),
        })
    }

    /// Returns the deserialized copy of a state object from the inner map
    pub async fn entry<S: AsRef<str>, T: DeserializeOwned>(&self, key: S) -> RepoResult<Option<T>> {
        let entries = self.inner.read().await;
        let entry = entries.get(key.as_ref());

        if let Some(bytes) = entry {
            let value = bincode::deserialize(bytes)?;

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Stores an entry in inner map
    pub async fn store_entry<T: Serialize>(&self, key: String, value: &T) -> RepoResult<()> {
        let entry_bytes = bincode::serialize(value)?;
        let mut entries = self.inner.write().await;
        entries.insert(key.clone(), entry_bytes);
        let mut changed_entries = self.changed_keys.write().await;
        changed_entries.insert(key);

        Ok(())
    }

    /// Returns a list of all changed state objects as an upsert list
    pub async fn changed_states(&self) -> Vec<UpsertJobStateDto> {
        let mut upsert_list = Vec::new();

        {
            let changed_keys = self.changed_keys.read().await;
            let entries = self.inner.read().await;

            for key in &*changed_keys {
                if let Some(value) = entries.get(key) {
                    upsert_list.push(UpsertJobStateDto {
                        job_id: self.job_id,
                        key: key.to_owned(),
                        value: value.clone(),
                    });
                }
            }
        }
        {
            let mut changed_keys = self.changed_keys.write().await;
            changed_keys.clear();
        }

        upsert_list
    }
}
