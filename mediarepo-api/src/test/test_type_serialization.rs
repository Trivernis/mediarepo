use crate::types::files::GetFileThumbnailOfSizeRequest;
use crate::types::filtering::{
    FilterExpression, FilterQuery, SortDirection, SortKey, TagQuery, ValueComparator,
};
use crate::types::identifier::FileIdentifier;
use bromine::payload::DynamicSerializer;
use bromine::prelude::IPCResult;
use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[test]
fn it_serializes_file_identifier() {
    test_serialization(FileIdentifier::ID(0)).unwrap();
}

#[test]
fn it_serializes_get_file_thumbnail_of_size_requests() {
    test_serialization(GetFileThumbnailOfSizeRequest {
        id: FileIdentifier::ID(0),
        max_size: (u32::MAX, u32::MAX),
        min_size: (0, 0),
    })
    .unwrap();
}

#[test]
fn it_serializes_tag_queries() {
    test_serialization(TagQuery {
        tag: String::from("Hello"),
        negate: true,
    })
    .unwrap();
}

#[test]
fn it_serializes_filter_expressions() {
    test_serialization(FilterExpression::Query(FilterQuery::Tag(TagQuery {
        tag: String::from("World"),
        negate: false,
    })))
    .unwrap();
}

#[test]
fn it_serializes_sort_keys() {
    test_serialization(SortKey::FileName(SortDirection::Descending)).unwrap();
}

#[test]
fn it_serializes_value_comparators() {
    test_serialization(ValueComparator::Between((
        NaiveDateTime::from_timestamp(100, 0),
        NaiveDateTime::from_timestamp(100, 10),
    )))
    .unwrap();
}

fn test_serialization<T: Serialize + DeserializeOwned>(data: T) -> IPCResult<()> {
    let serializer = DynamicSerializer::first_available();
    let bytes = serializer.serialize(data)?;
    let _: T = serializer.deserialize(&bytes[..])?;

    Ok(())
}
