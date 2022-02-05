use std::time::Duration;
use bromine::prelude::*;
use crate::client_api::error::ApiResult;
use crate::types::filtering::{SortingPreset, SortKey};
use super::IPCApi;

#[derive(Clone)]
pub struct PresetApi {
    ctx: PooledContext,
}

impl IPCApi for PresetApi {
    fn namespace() -> &'static str {
        "presets"
    }

    fn ctx(&self) -> PoolGuard<Context> {
        self.ctx.acquire()
    }
}

impl PresetApi {
    pub fn new(ctx: PooledContext) -> Self {
        Self { ctx }
    }

    /// Returns all sorting presets of the repository
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_sorting_presets(&self) -> ApiResult<Vec<SortingPreset>> {
        self.emit_and_get(
            "all_sorting_presets",
            (),
            Some(Duration::from_secs(1))
        )
            .await
    }

    /// Adds a new sorting preset with the given keys
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_sorting_preset(&self, keys: Vec<SortKey>) -> ApiResult<SortingPreset> {
        self.emit_and_get(
            "add_sorting_preset",
            keys,
            Some(Duration::from_secs(1))
        )
            .await
    }

    /// Deletes a given sorting preset by id
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete_sorting_preset(&self, id: i32) -> ApiResult<()> {
        self.emit_and_get("delete_sorting_preset", id, Some(Duration::from_secs(1))).await
    }
}
