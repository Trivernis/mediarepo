export type FilterExpression =
    { OrExpression: TagQuery[] }
    | { Query: TagQuery };

export type TagQuery = {
    negate: boolean,
    tag: string,
};

export type SortKey = { Namespace: SortNamespace }
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
    creation_time: Date,
    change_time: Date,
    import_time: Date,
};

export type FileOsMetadata = {
    name: string,
    path: string,
    mime_type: string,
    created_at: Date,
    modified_at: Date,
};
