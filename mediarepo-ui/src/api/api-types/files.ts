export type FilterExpression = FilterExpressionOrExpression | FilterExpressionQuery;

export type FilterExpressionOrExpression = {
    OrExpression: FilterQuery[],
};
export type FilterExpressionQuery = {
    Query: FilterQuery;
};

export type FilterQuery = FilterQueryTag | FilterQueryProperty;

export type FilterQueryTag = { Tag: TagQuery };
export type FilterQueryProperty = { Property: PropertyQuery };

export type TagQuery = {
    negate: boolean,
    tag: string,
};

export type PropertyQuery = PropertyQueryStatus
    | PropertyQueryFileSize
    | PropertyQueryImportedTime
    | PropertyQueryChangedTime
    | PropertyQueryCreatedTime
    | PropertyQueryTagCount
    | PropertyQueryCd
    | PropertyQueryId;

export type PropertyQueryStatus = { Status: FileStatus };
export type PropertyQueryFileSize = { FileSize: ValueComparator<number> };
export type PropertyQueryImportedTime = { ImportedTime: ValueComparator<Date> };
export type PropertyQueryChangedTime = { ChangedTime: ValueComparator<Date> };
export type PropertyQueryCreatedTime = { CreatedTime: ValueComparator<Date> };
export type PropertyQueryTagCount = { TagCount: ValueComparator<number> };
export type PropertyQueryCd = { Cd: string };
export type PropertyQueryId = { Id: number };

export type ValueComparator<T> =
    { Less: T }
    | { Equal: T }
    | { Greater: T }
    | { Between: T[] }

export type SortKeyData = { Namespace: SortNamespace }
    | { FileName: SortDirection }
    | { FileSize: SortDirection }
    | { FileImportedTime: SortDirection }
    | { FileChangeTime: SortDirection }
    | { FileType: SortDirection };

export type SortNamespace = {
    name: string,
    direction: SortDirection,
}

export type SortDirection = "Ascending" | "Descending";

export type FileBasicData = {
    id: number,
    status: FileStatus,
    cd: string,
    mime_type: string,
};

export type FileStatus = "Imported" | "Archived" | "Deleted";

export type FileMetadata = {
    file_id: number,
    name?: string,
    comment?: string,
    creation_time: string,
    change_time: string,
    import_time: string,
    size: number,
};

export type FileOsMetadata = {
    name: string,
    path: string,
    mime_type: string,
    created_at: Date,
    modified_at: Date,
};
