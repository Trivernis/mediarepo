use mediarepo_core::bromine::ipc::context::Context;
use mediarepo_core::content_descriptor::decode_content_descriptor;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::mediarepo_api::types::identifier::FileIdentifier;
use mediarepo_model::file::File;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use std::sync::Arc;

pub async fn get_repo_from_context(ctx: &Context) -> Arc<Repo> {
    let data = ctx.data.read().await;
    let repo = data.get::<RepoKey>().unwrap();
    Arc::clone(repo)
}

pub async fn file_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<File> {
    let file = match identifier {
        FileIdentifier::ID(id) => repo.file_by_id(id).await,
        FileIdentifier::CD(cd) => repo.file_by_cd(&decode_content_descriptor(cd)?).await,
    }?;
    file.ok_or_else(|| RepoError::from("File not found"))
}

pub async fn cd_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<Vec<u8>> {
    match identifier {
        FileIdentifier::ID(id) => {
            let file = repo
                .file_by_id(id)
                .await?
                .ok_or_else(|| "Thumbnail not found")?;
            Ok(file.cd().to_owned())
        }
        FileIdentifier::CD(cd) => decode_content_descriptor(cd),
    }
}
