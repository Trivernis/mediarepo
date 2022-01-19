use mediarepo_core::bromine::ipc::context::Context;
use mediarepo_core::content_descriptor::decode_content_descriptor;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::mediarepo_api::types::identifier::FileIdentifier;
use mediarepo_core::mediarepo_api::types::repo::SizeType;
use mediarepo_core::type_keys::{RepoPathKey, SettingsKey};
use mediarepo_core::utils::get_folder_size;
use mediarepo_logic::file::File;
use mediarepo_logic::repo::Repo;
use mediarepo_logic::type_keys::RepoKey;
use std::sync::Arc;
use tokio::fs;

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

pub async fn calculate_size(size_type: &SizeType, ctx: &Context) -> RepoResult<u64> {
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
            let db_path = settings.paths.db_file_path(&repo_path);

            let database_metadata = fs::metadata(db_path).await?;
            database_metadata.len()
        }
    };

    Ok(size)
}
