use mediarepo_core::content_descriptor::decode_content_descriptor;
use mediarepo_core::error::RepoResult;
use mediarepo_core::mediarepo_api::types::files::FileStatus as ApiFileStatus;
use mediarepo_core::mediarepo_api::types::filtering::{
    FilterExpression, FilterQuery, PropertyQuery, TagQuery, ValueComparator,
};
use mediarepo_logic::dao::file::find::NegatableComparator::{Is, IsNot};
use mediarepo_logic::dao::file::find::{FilterFileProperty, FilterProperty, OrderingComparator};
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use mediarepo_logic::dto::{FileDto, FileStatus};
use std::collections::HashMap;

#[tracing::instrument(level = "debug", skip(repo))]
pub async fn find_files_for_filters(
    repo: &Repo,
    expressions: Vec<FilterExpression>,
) -> RepoResult<Vec<FileDto>> {
    let tag_names = get_tag_names_from_expressions(&expressions);
    let tag_id_map = repo.tag_names_to_ids(tag_names).await?;
    let filters = build_filters_from_expressions(expressions, &tag_id_map);

    repo.file().find(filters).await
}

#[tracing::instrument(level = "debug")]
fn get_tag_names_from_expressions(expressions: &Vec<FilterExpression>) -> Vec<String> {
    expressions
        .iter()
        .flat_map(|f| match f {
            FilterExpression::OrExpression(queries) => queries
                .iter()
                .filter_map(|q| match q {
                    FilterQuery::Tag(tag) => Some(tag.tag.to_owned()),
                    _ => None,
                })
                .collect::<Vec<String>>(),
            FilterExpression::Query(q) => match q {
                FilterQuery::Tag(tag) => {
                    vec![tag.tag.to_owned()]
                }
                FilterQuery::Property(_) => {
                    vec![]
                }
            },
        })
        .collect::<Vec<String>>()
}

#[tracing::instrument(level = "debug")]
fn build_filters_from_expressions(
    expressions: Vec<FilterExpression>,
    tag_id_map: &HashMap<String, i64>,
) -> Vec<Vec<FilterProperty>> {
    expressions
        .into_iter()
        .filter_map(|e| {
            let filters = match e {
                FilterExpression::OrExpression(queries) => queries
                    .into_iter()
                    .filter_map(|q| map_query_to_filter(q, tag_id_map))
                    .collect(),
                FilterExpression::Query(q) => {
                    if let Some(filter) = map_query_to_filter(q, tag_id_map) {
                        vec![filter]
                    } else {
                        vec![]
                    }
                }
            };
            if filters.len() > 0 {
                Some(filters)
            } else {
                None
            }
        })
        .collect()
}

fn map_query_to_filter(
    query: FilterQuery,
    tag_id_map: &HashMap<String, i64>,
) -> Option<FilterProperty> {
    match query {
        FilterQuery::Tag(tag_query) => map_tag_query_to_filter(tag_query, tag_id_map),
        FilterQuery::Property(property) => map_property_query_to_filter(property),
    }
}

fn map_tag_query_to_filter(
    query: TagQuery,
    tag_id_map: &HashMap<String, i64>,
) -> Option<FilterProperty> {
    if query.tag.ends_with("*") {
        map_wildcard_tag_to_filter(query, tag_id_map)
    } else {
        map_tag_to_filter(query, tag_id_map)
    }
}

fn map_wildcard_tag_to_filter(
    query: TagQuery,
    tag_id_map: &HashMap<String, i64>,
) -> Option<FilterProperty> {
    let filter_tag = query.tag.trim_end_matches("*");
    let relevant_ids = tag_id_map
        .iter()
        .filter_map(|(name, id)| {
            if name.starts_with(filter_tag) {
                Some(*id)
            } else {
                None
            }
        })
        .collect::<Vec<i64>>();

    if relevant_ids.len() > 0 {
        let comparator = if query.negate {
            IsNot(relevant_ids)
        } else {
            Is(relevant_ids)
        };
        Some(FilterProperty::TagWildcardIds(comparator))
    } else {
        None
    }
}

fn map_tag_to_filter(query: TagQuery, tag_id_map: &HashMap<String, i64>) -> Option<FilterProperty> {
    tag_id_map.get(&query.tag).map(|id| {
        let comparator = if query.negate { IsNot(*id) } else { Is(*id) };
        FilterProperty::TagId(comparator)
    })
}

fn map_property_query_to_filter(query: PropertyQuery) -> Option<FilterProperty> {
    match query {
        PropertyQuery::Status(s) => Some(FilterProperty::FileProperty(FilterFileProperty::Status(
            Is(file_status_to_number(s)),
        ))),
        PropertyQuery::FileSize(s) => Some(FilterProperty::FileProperty(
            FilterFileProperty::FileSize(val_comparator_to_order(s, |v| v as i64)),
        )),
        PropertyQuery::ImportedTime(t) => Some(FilterProperty::FileProperty(
            FilterFileProperty::ImportedTime(val_comparator_to_order(t, |t| t)),
        )),
        PropertyQuery::ChangedTime(t) => Some(FilterProperty::FileProperty(
            FilterFileProperty::ChangedTime(val_comparator_to_order(t, |t| t)),
        )),
        PropertyQuery::CreatedTime(t) => Some(FilterProperty::FileProperty(
            FilterFileProperty::CreatedTime(val_comparator_to_order(t, |t| t)),
        )),
        PropertyQuery::TagCount(c) => {
            Some(FilterProperty::TagCount(val_comparator_to_order(c, |v| {
                v as i64
            })))
        }
        PropertyQuery::Cd(cd) => decode_content_descriptor(cd)
            .ok()
            .map(|cd| FilterProperty::ContentDescriptor(Is(cd))),
        PropertyQuery::Id(id) => Some(FilterProperty::FileProperty(FilterFileProperty::Id(Is(id)))),
    }
}

fn file_status_to_number(status: ApiFileStatus) -> i64 {
    match status {
        ApiFileStatus::Imported => FileStatus::Imported as i64,
        ApiFileStatus::Archived => FileStatus::Archived as i64,
        ApiFileStatus::Deleted => FileStatus::Deleted as i64,
    }
}

#[inline]
fn val_comparator_to_order<T1, T2, F: Fn(T1) -> T2>(
    comp: ValueComparator<T1>,
    conv_fn: F,
) -> OrderingComparator<T2> {
    match comp {
        ValueComparator::Less(v) => OrderingComparator::Less(conv_fn(v)),
        ValueComparator::Equal(v) => OrderingComparator::Equal(conv_fn(v)),
        ValueComparator::Greater(v) => OrderingComparator::Greater(conv_fn(v)),
        ValueComparator::Between((v1, v2)) => {
            OrderingComparator::Between((conv_fn(v1), conv_fn(v2)))
        }
    }
}
