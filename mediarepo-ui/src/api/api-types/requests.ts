import {FileOsMetadata, FileStatus, FilterExpression, SortKeyData} from "./files";
import {RepositoryData, SizeType} from "./repo";
import {JobType} from "./job";

type NameIdentifierRequest = {
    name: string
};

type IdIdentifierRequest = {
    id: number
};

type RepoPathIdentifier = {
    repoPath: string;
}

export type SelectRepositoryRequest = NameIdentifierRequest;

export type AddRepositoryRequest = RepositoryData;

export type CheckLocalRepositoryExistsRequest = {
    path: string
};

export type RemoveRepositoryRequest = NameIdentifierRequest;

export type DeleteRepositoryRequest = NameIdentifierRequest;

export type CheckDaemonRunningRequest = {
    address: string
};

export type StartDaemonRequest = RepoPathIdentifier;

export type InitRepositoryRequest = RepoPathIdentifier;

export type GetSizeRequest = {
    sizeType: SizeType
};

export type FindFilesRequest = {
    filters: FilterExpression[],
    sortBy: SortKeyData[]
};

export type UpdateFileNameRequest = {
    id: number,
    name: string,
};

export type SaveFileRequest = {
    id: number,
    path: string,
};

export type DeleteThumbnailsRequest = IdIdentifierRequest;

export type ReadFileRequest = {
    hash: string,
    mimeType: string,
};

export type DeleteFileRequest = IdIdentifierRequest;

export type GetFileMetadataRequest = IdIdentifierRequest;

export type UpdateFileStatusRequest = {
    id: number,
    status: FileStatus
};

export type GetTagsForFilesRequest = {
    cds: string[]
};

export type GetFileTagMapRequest = {
    cds: string[]
};

export type CreateTagsRequest = {
    tags: string[]
};

export type ChangeFileTagsRequest = {
    id: number,
    addedTags: number[],
    removedTags: number[],
};

export type ResolvePathsToFilesRequest = {
    paths: string[],
};

export type AddLocalFileREquest = {
    metadata: FileOsMetadata,
    options: AddFileOptions,
}

type AddFileOptions = {
    read_tags_from_txt: boolean,
    delete_after_import: boolean,
};

export type SetFrontendStateRequest = {
    state: string
};

export type RunJobRequest = {
    jobType: JobType,
    sync: boolean,
};

export type AddSortingPresetRequest = {
    sortKeys: SortKeyData[]
};

export type DeleteSortingPresetRequest = {
    id: number
};

export type IsJobRunningRequest = {
    jobType: JobType,
}
