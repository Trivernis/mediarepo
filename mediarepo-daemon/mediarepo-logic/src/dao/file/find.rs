use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use sea_orm::Condition;
use sea_orm::sea_query::{Alias, Expr, Query, SimpleExpr};

use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::content_descriptor;
use mediarepo_database::entities::content_descriptor_tag;
use mediarepo_database::entities::file;
use mediarepo_database::entities::file_metadata;

use crate::dao::file::{FileDao, map_cd_and_file};
use crate::dto::FileDto;

macro_rules! apply_ordering_comparator {
    ($column:expr, $filter:expr) => {
        match $filter {
            OrderingComparator::Less(value) => $column.lt(value),
            OrderingComparator::Equal(value) => $column.eq(value),
            OrderingComparator::Greater(value) => $column.gt(value),
            OrderingComparator::Between((min_value, max_value)) => {
                $column.between(min_value, max_value)
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum FilterProperty {
    TagId(NegatableComparator<i64>),
    TagWildcardIds(NegatableComparator<Vec<i64>>),
    ContentDescriptor(NegatableComparator<Vec<u8>>),
    TagCount(OrderingComparator<i64>),
    FileProperty(FilterFileProperty),
}

#[derive(Clone, Debug)]
pub enum FilterFileProperty {
    Id(NegatableComparator<i64>),
    Status(NegatableComparator<i64>),
    FileSize(OrderingComparator<i64>),
    ImportedTime(OrderingComparator<NaiveDateTime>),
    ChangedTime(OrderingComparator<NaiveDateTime>),
    CreatedTime(OrderingComparator<NaiveDateTime>),
}

#[derive(Clone, Debug)]
pub enum OrderingComparator<T> {
    Less(T),
    Equal(T),
    Greater(T),
    Between((T, T)),
}

#[derive(Clone, Debug)]
pub enum NegatableComparator<T> {
    Is(T),
    IsNot(T),
}

impl FileDao {
    /// Finds files by filters
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn find(&self, filters: Vec<Vec<FilterProperty>>) -> RepoResult<Vec<FileDto>> {
        let main_condition = build_find_filter_conditions(filters);

        let files = content_descriptor::Entity::find()
            .find_also_related(file::Entity)
            .filter(main_condition)
            .group_by(file::Column::Id)
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .filter_map(map_cd_and_file)
            .collect();

        Ok(files)
    }
}

#[tracing::instrument(level = "debug")]
fn build_find_filter_conditions(filters: Vec<Vec<FilterProperty>>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |all_cond, mut expression| {
            if expression.len() == 1 {
                let property = expression.pop().unwrap();

                all_cond.add(build_single_filter(property))
            } else if !expression.is_empty() {
                let sub_condition = expression.into_iter().fold(Condition::any(), |cond, prop| {
                    cond.add(build_single_filter(prop))
                });

                all_cond.add(sub_condition)
            } else {
                all_cond
            }
        })
}

#[inline]
fn build_single_filter(property: FilterProperty) -> SimpleExpr {
    match property {
        FilterProperty::TagId(tag_filter) => build_tag_id_filter(tag_filter),
        FilterProperty::TagWildcardIds(wildcard_filter) => {
            build_tag_wildcard_ids_filter(wildcard_filter)
        }
        FilterProperty::ContentDescriptor(cd_filter) => build_content_descriptor_filter(cd_filter),
        FilterProperty::TagCount(count_filter) => build_tag_count_filter(count_filter),
        FilterProperty::FileProperty(property_filter) => {
            build_file_property_filter(property_filter)
        }
    }
}

fn build_tag_id_filter(filter: NegatableComparator<i64>) -> SimpleExpr {
    match filter {
        NegatableComparator::Is(tag_id) => content_descriptor::Column::Id.in_subquery(
            Query::select()
                .expr(Expr::col(content_descriptor_tag::Column::CdId))
                .from(content_descriptor_tag::Entity)
                .cond_where(content_descriptor_tag::Column::TagId.eq(tag_id))
                .to_owned(),
        ),
        NegatableComparator::IsNot(tag_id) => content_descriptor::Column::Id.not_in_subquery(
            Query::select()
                .expr(Expr::col(content_descriptor_tag::Column::CdId))
                .from(content_descriptor_tag::Entity)
                .cond_where(content_descriptor_tag::Column::TagId.eq(tag_id))
                .to_owned(),
        ),
    }
}

fn build_tag_wildcard_ids_filter(filter: NegatableComparator<Vec<i64>>) -> SimpleExpr {
    match filter {
        NegatableComparator::Is(tag_ids) => content_descriptor::Column::Id.in_subquery(
            Query::select()
                .expr(Expr::col(content_descriptor_tag::Column::CdId))
                .from(content_descriptor_tag::Entity)
                .cond_where(content_descriptor_tag::Column::TagId.is_in(tag_ids))
                .to_owned(),
        ),
        NegatableComparator::IsNot(tag_ids) => content_descriptor::Column::Id.not_in_subquery(
            Query::select()
                .expr(Expr::col(content_descriptor_tag::Column::CdId))
                .from(content_descriptor_tag::Entity)
                .cond_where(content_descriptor_tag::Column::TagId.is_in(tag_ids))
                .to_owned(),
        ),
    }
}

fn build_content_descriptor_filter(filter: NegatableComparator<Vec<u8>>) -> SimpleExpr {
    match filter {
        NegatableComparator::Is(cd) => content_descriptor::Column::Descriptor.eq(cd),
        NegatableComparator::IsNot(cd) => content_descriptor::Column::Descriptor.ne(cd),
    }
}

fn build_tag_count_filter(filter: OrderingComparator<i64>) -> SimpleExpr {
    let count_column = Alias::new("count");
    let cd_id_column = Alias::new("cd_id");

    let count_subquery = Query::select()
        .expr_as(
            Expr::col(content_descriptor_tag::Column::CdId),
            cd_id_column.clone(),
        )
        .expr_as(
            content_descriptor_tag::Column::TagId.count(),
            count_column.clone(),
        )
        .from(content_descriptor_tag::Entity)
        .group_by_col(cd_id_column.clone())
        .to_owned();

    let count_expression = apply_ordering_comparator!(Expr::col(count_column), filter);

    content_descriptor::Column::Id.in_subquery(
        Query::select()
            .expr(Expr::col(cd_id_column))
            .from_subquery(count_subquery, Alias::new("tag_counts"))
            .cond_where(count_expression)
            .to_owned(),
    )
}

#[inline]
fn build_file_property_filter(property: FilterFileProperty) -> SimpleExpr {
    match property {
        FilterFileProperty::Id(id_filter) => build_file_id_filter(id_filter),
        FilterFileProperty::Status(status_filter) => build_file_status_filter(status_filter),
        FilterFileProperty::FileSize(size_filter) => {
            build_file_metadata_filter(build_file_size_filter(size_filter))
        }
        FilterFileProperty::ImportedTime(time_filter) => {
            build_file_metadata_filter(build_file_import_time_filter(time_filter))
        }
        FilterFileProperty::ChangedTime(time_filter) => {
            build_file_metadata_filter(build_file_changed_time_filter(time_filter))
        }
        FilterFileProperty::CreatedTime(time_filter) => {
            build_file_metadata_filter(build_file_created_time_filter(time_filter))
        }
    }
}

fn build_file_id_filter(filter: NegatableComparator<i64>) -> SimpleExpr {
    match filter {
        NegatableComparator::Is(id) => file::Column::Id.eq(id),
        NegatableComparator::IsNot(id) => file::Column::Id.ne(id),
    }
}

fn build_file_status_filter(filter: NegatableComparator<i64>) -> SimpleExpr {
    match filter {
        NegatableComparator::Is(status) => file::Column::Status.eq(status),
        NegatableComparator::IsNot(status) => file::Column::Status.ne(status),
    }
}

fn build_file_metadata_filter(property_condition: SimpleExpr) -> SimpleExpr {
    file::Column::Id.in_subquery(
        Query::select()
            .expr(Expr::col(file_metadata::Column::FileId))
            .from(file_metadata::Entity)
            .cond_where(property_condition)
            .to_owned(),
    )
}

fn build_file_size_filter(filter: OrderingComparator<i64>) -> SimpleExpr {
    apply_ordering_comparator!(file_metadata::Column::Size, filter)
}

fn build_file_import_time_filter(filter: OrderingComparator<NaiveDateTime>) -> SimpleExpr {
    apply_ordering_comparator!(file_metadata::Column::ImportTime, filter)
}

fn build_file_changed_time_filter(filter: OrderingComparator<NaiveDateTime>) -> SimpleExpr {
    apply_ordering_comparator!(file_metadata::Column::ChangeTime, filter)
}

fn build_file_created_time_filter(filter: OrderingComparator<NaiveDateTime>) -> SimpleExpr {
    apply_ordering_comparator!(file_metadata::Column::CreationTime, filter)
}
