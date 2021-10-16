use mediarepo_core::rmp_ipc::ipc::context::Context;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use std::sync::Arc;

pub async fn get_repo_from_context(ctx: &Context) -> Arc<Repo> {
    let data = ctx.data.read().await;
    let repo = data.get::<RepoKey>().unwrap();
    Arc::clone(repo)
}
