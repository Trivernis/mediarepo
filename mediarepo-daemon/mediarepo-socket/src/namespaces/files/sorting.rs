use chrono::NaiveDateTime;
use compare::Compare;
use mediarepo_core::error::RepoResult;
use mediarepo_core::mediarepo_api::types::filtering::{SortDirection, SortKey};
use mediarepo_database::queries::tags::{
    get_cids_with_namespaced_tags, get_content_descriptors_with_tag_count,
};
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use mediarepo_logic::dto::{FileDto, FileMetadataDto};
use mediarepo_logic::file::File;
use mediarepo_logic::file_metadata::FileMetadata;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct FileSortContext {
    name: Option<String>,
    size: u64,
    mime_type: String,
    namespaces: HashMap<String, Vec<String>>,
    tag_count: u32,
    import_time: NaiveDateTime,
    create_time: NaiveDateTime,
    change_time: NaiveDateTime,
}

#[tracing::instrument(level = "debug", skip(repo, files))]
pub async fn sort_files_by_properties(
    repo: &Repo,
    sort_expression: Vec<SortKey>,
    files: &mut Vec<FileDto>,
) -> RepoResult<()> {
    let contexts = build_sort_context(repo, files).await?;

    files.sort_by(|a, b| {
        compare_files(
            contexts.get(&a.id()).unwrap(),
            contexts.get(&b.id()).unwrap(),
            &sort_expression,
        )
    });

    Ok(())
}

async fn build_sort_context(
    repo: &Repo,
    files: &Vec<FileDto>,
) -> RepoResult<HashMap<i64, FileSortContext>> {
    let hash_ids: Vec<i64> = files.par_iter().map(|f| f.cd_id()).collect();
    let file_ids: Vec<i64> = files.par_iter().map(|f| f.id()).collect();

    let mut cid_nsp: HashMap<i64, HashMap<String, Vec<String>>> =
        get_cids_with_namespaced_tags(repo.db(), hash_ids.clone()).await?;
    let mut cid_tag_counts = get_content_descriptors_with_tag_count(repo.db(), hash_ids).await?;

    let files_metadata = repo.file().all_metadata(file_ids).await?;

    let mut file_metadata_map: HashMap<i64, FileMetadataDto> =
        HashMap::from_iter(files_metadata.into_iter().map(|m| (m.file_id(), m)));

    let mut contexts = HashMap::new();

    for file in files {
        if let Some(metadata) = file_metadata_map.remove(&file.id()) {
            let context = FileSortContext {
                name: metadata.name().cloned(),
                size: metadata.size() as u64,
                mime_type: file.mime_type().to_owned(),
                namespaces: cid_nsp
                    .remove(&file.cd_id())
                    .unwrap_or(HashMap::with_capacity(0)),
                tag_count: cid_tag_counts.remove(&file.cd_id()).unwrap_or(0),
                import_time: metadata.import_time().to_owned(),
                create_time: metadata.import_time().to_owned(),
                change_time: metadata.change_time().to_owned(),
            };
            contexts.insert(file.id(), context);
        }
    }

    Ok(contexts)
}

#[tracing::instrument(level = "trace", skip_all)]
fn compare_files(
    ctx_a: &FileSortContext,
    ctx_b: &FileSortContext,
    expression: &Vec<SortKey>,
) -> Ordering {
    let cmp_date = compare::natural();
    let cmp_u64 = compare::natural();
    let cmp_u32 = compare::natural();

    for sort_key in expression {
        let ordering = match sort_key {
            SortKey::Namespace(namespace) => {
                let list_a = ctx_a.namespaces.get(&namespace.name);
                let list_b = ctx_b.namespaces.get(&namespace.name);

                let cmp_result = if let (Some(list_a), Some(list_b)) = (list_a, list_b) {
                    compare_tag_lists(list_a, list_b)
                } else if list_a.is_some() {
                    Ordering::Greater
                } else if list_b.is_some() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                };
                adjust_for_dir(cmp_result, &namespace.direction)
            }
            SortKey::FileName(direction) => {
                adjust_for_dir(compare_opts(&ctx_a.name, &ctx_b.name), direction)
            }
            SortKey::FileSize(direction) => {
                adjust_for_dir(cmp_u64.compare(&ctx_a.size, &ctx_b.size), direction)
            }
            SortKey::FileImportedTime(direction) => adjust_for_dir(
                cmp_date.compare(&ctx_a.import_time, &ctx_b.import_time),
                direction,
            ),
            SortKey::FileCreatedTime(direction) => adjust_for_dir(
                cmp_date.compare(&ctx_a.create_time, &ctx_b.create_time),
                direction,
            ),
            SortKey::FileChangeTime(direction) => adjust_for_dir(
                cmp_date.compare(&ctx_a.change_time, &ctx_b.change_time),
                direction,
            ),
            SortKey::FileType(direction) => {
                adjust_for_dir(ctx_a.mime_type.cmp(&ctx_b.mime_type), direction)
            }
            SortKey::NumTags(direction) => adjust_for_dir(
                cmp_u32.compare(&ctx_a.tag_count, &ctx_b.tag_count),
                direction,
            ),
        };
        if !ordering.is_eq() {
            return ordering;
        }
    }

    Ordering::Equal
}

fn compare_opts<T: Ord + Sized>(opt_a: &Option<T>, opt_b: &Option<T>) -> Ordering {
    let cmp = compare::natural();
    if let (Some(a), Some(b)) = (opt_a, opt_b) {
        cmp.compare(a, b)
    } else if opt_a.is_some() {
        Ordering::Greater
    } else if opt_b.is_some() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn compare_f32(a: f32, b: f32) -> Ordering {
    if a > b {
        Ordering::Greater
    } else if b > a {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn adjust_for_dir(ordering: Ordering, direction: &SortDirection) -> Ordering {
    if *direction == SortDirection::Descending {
        ordering.reverse()
    } else {
        ordering
    }
}

fn compare_tag_lists(list_a: &Vec<String>, list_b: &Vec<String>) -> Ordering {
    let first_diff = list_a
        .into_iter()
        .zip(list_b.into_iter())
        .find(|(a, b)| *a != *b);
    if let Some(diff) = first_diff {
        if let (Some(num_a), Some(num_b)) = (diff.0.parse::<f32>().ok(), diff.1.parse::<f32>().ok())
        {
            compare_f32(num_a, num_b)
        } else {
            let cmp = compare::natural();
            cmp.compare(diff.0, diff.1)
        }
    } else {
        Ordering::Equal
    }
}
