use std::sync::Arc;

use crate::TypeMap;
use mediarepo_core::bromine::ipc::context::Context;
use mediarepo_core::content_descriptor::decode_content_descriptor;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::mediarepo_api::types::identifier::FileIdentifier;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use mediarepo_logic::dto::FileDto;
use mediarepo_logic::type_keys::RepoKey;
use mediarepo_worker::job_dispatcher::{DispatcherKey, JobDispatcher};

pub async fn get_repo_from_context(ctx: &Context) -> Arc<Repo> {
    let data = ctx.data.read().await;
    let repo = data.get::<RepoKey>().unwrap();
    Arc::clone(repo)
}

pub async fn get_job_dispatcher_from_context(ctx: &Context) -> JobDispatcher {
    let data = ctx.data.read().await;
    data.get::<DispatcherKey>().unwrap().clone()
}

pub async fn file_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<FileDto> {
    let file = match identifier {
        FileIdentifier::ID(id) => repo.file().by_id(id).await,
        FileIdentifier::CD(cd) => repo.file().by_cd(decode_content_descriptor(cd)?).await,
    }?;
    file.ok_or_else(|| RepoError::from("File not found"))
}

pub async fn cd_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<Vec<u8>> {
    match identifier {
        FileIdentifier::ID(id) => {
            let file = repo.file().by_id(id).await?.ok_or("Thumbnail not found")?;
            Ok(file.cd().to_owned())
        }
        FileIdentifier::CD(cd) => decode_content_descriptor(cd),
    }
}
