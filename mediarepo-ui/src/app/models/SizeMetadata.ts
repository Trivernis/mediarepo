export enum SizeType {
    Total = "Total",
    FileFolder = "FileFolder",
    ThumbFolder = "ThumbFolder",
    DatabaseFile = "DatabaseFile",
}

export type SizeMetadata = {
    size_type: SizeType,
    size: number,
}
