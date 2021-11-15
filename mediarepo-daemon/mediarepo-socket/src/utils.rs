use mediarepo_api::types::identifier::FileIdentifier;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::rmp_ipc::ipc::context::Context;
use mediarepo_core::rmp_ipc::protocol::AsyncProtocolStream;
use mediarepo_model::file::File;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use std::sync::Arc;

pub async fn get_repo_from_context<S: AsyncProtocolStream>(ctx: &Context<S>) -> Arc<Repo> {
    let data = ctx.data.read().await;
    let repo = data.get::<RepoKey>().unwrap();
    Arc::clone(repo)
}

pub async fn file_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<File> {
    let file = match identifier {
        FileIdentifier::ID(id) => repo.file_by_id(id).await,
        FileIdentifier::Hash(hash) => repo.file_by_hash(hash).await,
    }?;
    file.ok_or_else(|| RepoError::from("Thumbnail not found"))
}

pub async fn hash_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<String> {
    match identifier {
        FileIdentifier::ID(id) => {
            let file = repo
                .file_by_id(id)
                .await?
                .ok_or_else(|| "Thumbnail not found")?;
            Ok(file.hash().to_owned())
        }
        FileIdentifier::Hash(hash) => Ok(hash),
    }
}
