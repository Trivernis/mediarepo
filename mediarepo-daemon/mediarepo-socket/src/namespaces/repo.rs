use std::path::PathBuf;

use tokio::fs;

use mediarepo_core::bromine::prelude::*;
use mediarepo_core::mediarepo_api::types::repo::{
    FrontendState, RepositoryMetadata, SizeMetadata, SizeType,
};
use mediarepo_core::type_keys::{RepoPathKey, SettingsKey};
use mediarepo_core::utils::get_folder_size;

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
    async fn get_metadata(ctx: &Context, _: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let counts = repo.get_counts().await?;

        let metadata = RepositoryMetadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            file_count: counts.file_count as u64,
            tag_count: counts.tag_count as u64,
            namespace_count: counts.namespace_count as u64,
            mapping_count: counts.mapping_count as u64,
            hash_count: counts.hash_count as u64,
        };

        tracing::debug!("metadata = {:?}", metadata);
        ctx.emit_to(Self::name(), "repository_metadata", metadata)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn get_size_metadata(ctx: &Context, event: Event) -> IPCResult<()> {
        let size_type = event.payload::<SizeType>()?;
        let repo = get_repo_from_context(ctx).await;
        let (repo_path, settings) = {
            let data = ctx.data.read().await;
            (
                data.get::<RepoPathKey>().unwrap().clone(),
                data.get::<SettingsKey>().unwrap().clone(),
            )
        };
        let size = match &size_type {
            SizeType::Total => get_folder_size(repo_path).await?,
            SizeType::FileFolder => repo.get_main_store_size().await?,
            SizeType::ThumbFolder => repo.get_thumb_store_size().await?,
            SizeType::DatabaseFile => {
                let db_path = repo_path.join(settings.database_path);

                let database_metadata = fs::metadata(db_path).await?;
                database_metadata.len()
            }
        };
        let response = SizeMetadata { size, size_type };
        tracing::debug!("size response = {:?}", response);

        ctx.emit_to(Self::name(), "size_metadata", response).await?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn frontend_state(ctx: &Context, _: Event) -> IPCResult<()> {
        let path = get_frontend_state_path(ctx).await?;
        let state_string = if path.exists() {
            Some(fs::read_to_string(path).await?)
        } else {
            None
        };
        ctx.emit_to(
            Self::name(),
            "frontend_state",
            FrontendState {
                state: state_string,
            },
        )
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn set_frontend_state(ctx: &Context, event: Event) -> IPCResult<()> {
        let path = get_frontend_state_path(ctx).await?;
        let state = event.payload::<FrontendState>()?.state;
        if let Some(state_string) = state {
            fs::write(path, state_string.into_bytes()).await?;
        } else {
            fs::remove_file(path).await?;
        }

        Ok(())
    }
}

async fn get_frontend_state_path(ctx: &Context) -> IPCResult<PathBuf> {
    let data = ctx.data.read().await;
    let settings = data.get::<SettingsKey>().unwrap();
    let repo_path = data.get::<RepoPathKey>().unwrap();
    let state_path = repo_path
        .join(PathBuf::from(&settings.database_path).parent().unwrap())
        .join("frontend-state.json");

    Ok(state_path)
}
