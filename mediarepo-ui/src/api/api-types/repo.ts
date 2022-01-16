export type RepositoryMetadata = {
    version: string,
    file_count: number,
    tag_count: number,
    namespace_count: number,
    mapping_count: number,
    hash_count: number,
};

export type SizeMetadata = {
    size_type: SizeType,
    size: number,
};

export type SizeType = "Total" | "FileFolder" | "ThumbFolder" | "DatabaseFile";

export type RepositoryData = {
    name: string,
    address?: string,
    path?: string,
    local: boolean,
}
