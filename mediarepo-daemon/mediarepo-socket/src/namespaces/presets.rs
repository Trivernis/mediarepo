use crate::from_model::FromModel;
use crate::utils::get_repo_from_context;
use mediarepo_core::bromine::prelude::*;
use mediarepo_core::mediarepo_api::types::filtering::{SortDirection, SortKey, SortingPreset};
use mediarepo_logic::dao::DaoProvider;
use mediarepo_logic::dto::{AddSortKeyDto, AddSortingPresetDto, KeyType};

pub struct PresetsNamespace;

impl NamespaceProvider for PresetsNamespace {
    fn name() -> &'static str {
        "presets"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "all_sorting_presets" => Self::all_sorting_presets,
            "add_sorting_preset" => Self::add_sorting_preset,
            "delete_sorting_preset" => Self::delete_sorting_preset
        );
    }
}

impl PresetsNamespace {
    #[tracing::instrument(skip_all)]
    pub async fn all_sorting_presets(ctx: &Context, _: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let sorting_presets: Vec<SortingPreset> = repo
            .sorting_preset()
            .all()
            .await?
            .into_iter()
            .map(SortingPreset::from_model)
            .collect();

        ctx.response(sorting_presets)
    }

    #[tracing::instrument(skip_all)]
    pub async fn add_sorting_preset(ctx: &Context, event: Event) -> IPCResult<Response> {
        let keys = event
            .payload::<Vec<SortKey>>()?
            .into_iter()
            .map(sort_key_to_add_dto)
            .collect();
        let repo = get_repo_from_context(ctx).await;
        let preset = repo
            .sorting_preset()
            .add(AddSortingPresetDto { keys })
            .await?;

        ctx.response(SortingPreset::from_model(preset))
    }

    #[tracing::instrument(skip_all)]
    pub async fn delete_sorting_preset(ctx: &Context, event: Event) -> IPCResult<Response> {
        let id = event.payload::<i32>()?;
        let repo = get_repo_from_context(ctx).await;
        repo.sorting_preset().delete(id).await?;

        Ok(Response::empty())
    }
}

fn sort_key_to_add_dto(key: SortKey) -> AddSortKeyDto {
    match key {
        SortKey::Namespace(namespace) => AddSortKeyDto {
            ascending: namespace.direction == SortDirection::Ascending,
            key_type: KeyType::Namespace,
            value: Some(namespace.name),
        },
        SortKey::FileName(dir) => AddSortKeyDto {
            ascending: dir == SortDirection::Ascending,
            key_type: KeyType::FileName,
            value: None,
        },
        SortKey::FileSize(dir) => AddSortKeyDto {
            ascending: dir == SortDirection::Ascending,
            key_type: KeyType::FileSize,
            value: None,
        },
        SortKey::FileImportedTime(dir) => AddSortKeyDto {
            ascending: dir == SortDirection::Ascending,
            key_type: KeyType::FileImportedTime,
            value: None,
        },
        SortKey::FileCreatedTime(dir) => AddSortKeyDto {
            ascending: dir == SortDirection::Ascending,
            key_type: KeyType::FileCreatedTime,
            value: None,
        },
        SortKey::FileChangeTime(dir) => AddSortKeyDto {
            ascending: dir == SortDirection::Ascending,
            key_type: KeyType::FileChangeTime,
            value: None,
        },
        SortKey::FileType(dir) => AddSortKeyDto {
            ascending: dir == SortDirection::Ascending,
            key_type: KeyType::FileType,
            value: None,
        },
        SortKey::NumTags(dir) => AddSortKeyDto {
            ascending: dir == SortDirection::Ascending,
            key_type: KeyType::NumTags,
            value: None,
        },
    }
}
