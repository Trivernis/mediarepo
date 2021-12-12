use mediarepo_core::bromine::prelude::*;
use mediarepo_core::mediarepo_api::types::repo::FrontendState;
use mediarepo_core::type_keys::{RepoPathKey, SettingsKey};
use std::path::PathBuf;
use tokio::fs;

pub struct RepoNamespace;

impl NamespaceProvider for RepoNamespace {
    fn name() -> &'static str {
        "repo"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "frontend_state" => Self::frontend_state,
            "set_frontend_state" => Self::set_frontend_state
        );
    }
}

impl RepoNamespace {
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
