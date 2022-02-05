use crate::types::files::FileStatus;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindFilesRequest {
    pub filters: Vec<FilterExpression>,
    pub sort_expression: Vec<SortKey>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FilterExpression {
    OrExpression(Vec<FilterQuery>),
    Query(FilterQuery),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FilterQuery {
    Tag(TagQuery),
    Property(PropertyQuery),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagQuery {
    pub negate: bool,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropertyQuery {
    Status(FileStatus),
    FileSize(ValueComparator<u64>),
    ImportedTime(ValueComparator<NaiveDateTime>),
    ChangedTime(ValueComparator<NaiveDateTime>),
    CreatedTime(ValueComparator<NaiveDateTime>),
    TagCount(ValueComparator<u64>),
    Cd(String),
    Id(i64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValueComparator<T> {
    Less(T),
    Equal(T),
    Greater(T),
    Between((T, T)),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SortKey {
    Namespace(SortNamespace),
    FileName(SortDirection),
    FileSize(SortDirection),
    FileImportedTime(SortDirection),
    FileCreatedTime(SortDirection),
    FileChangeTime(SortDirection),
    FileType(SortDirection),
    NumTags(SortDirection),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortNamespace {
    pub name: String,
    pub direction: SortDirection,
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Eq for SortDirection {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortingPreset {
    pub id: i32,
    pub keys: Vec<SortKey>,
}