use std::path::PathBuf;

use tokio::fs;

use mediarepo_core::bromine::prelude::*;
use mediarepo_core::mediarepo_api::types::repo::{
    FrontendState, RepositoryMetadata, SizeMetadata, SizeType,
};
use mediarepo_core::type_keys::{RepoPathKey, SettingsKey, SizeMetadataKey};

use crate::utils::get_repo_from_context;

pub struct RepoNamespace;

impl NamespaceProvider for RepoNamespace {
    fn name() -> &'static str {
        "repo"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "repository_metadata" => Self::get_metadata,
            "size_metadata" => Self::get_size_metadata,
            "frontend_state" => Self::frontend_state,
            "set_frontend_state" => Self::set_frontend_state
        );
    }
}

impl RepoNamespace {
    #[tracing::instrument(skip_all)]
    async fn get_metadata(ctx: &Context, _: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let counts = repo.get_counts().await?;

        let metadata = RepositoryMetadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            file_count: counts.file_count as u64,
            tag_count: counts.tag_count as u64,
            namespace_count: counts.namespace_count as u64,
            mapping_count: counts.mapping_count as u64,
            hash_count: counts.cd_count as u64,
        };

        tracing::debug!("metadata = {:?}", metadata);

        ctx.response(metadata)
    }

    #[tracing::instrument(skip_all)]
    async fn get_size_metadata(ctx: &Context, event: Event) -> IPCResult<Response> {
        let size_type = event.payload::<SizeType>()?;
        let data = ctx.data.read().await;
        let size_cache = data.get::<SizeMetadataKey>().unwrap();

        let size = if let Some(size) = size_cache.get(&size_type) {
            *size
        } else {
            0
        };

        ctx.response(SizeMetadata { size, size_type })
    }

    #[tracing::instrument(skip_all)]
    async fn frontend_state(ctx: &Context, _: Event) -> IPCResult<Response> {
        let path = get_frontend_state_path(ctx).await?;
        let state_string = if path.exists() {
            Some(fs::read_to_string(path).await?)
        } else {
            None
        };

        ctx.response(FrontendState {
            state: state_string,
        })
    }

    #[tracing::instrument(skip_all)]
    async fn set_frontend_state(ctx: &Context, event: Event) -> IPCResult<Response> {
        let path = get_frontend_state_path(ctx).await?;
        let state = event.payload::<FrontendState>()?.state;
        if let Some(state_string) = state {
            fs::write(path, state_string.into_bytes()).await?;
        } else {
            fs::remove_file(path).await?;
        }

        Ok(Response::empty())
    }
}

async fn get_frontend_state_path(ctx: &Context) -> IPCResult<PathBuf> {
    let data = ctx.data.read().await;
    let settings = data.get::<SettingsKey>().unwrap();
    let repo_path = data.get::<RepoPathKey>().unwrap();
    let state_path = settings.paths.frontend_state_file_path(&repo_path);

    Ok(state_path)
}
