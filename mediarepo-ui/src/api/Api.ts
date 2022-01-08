import {FileBasicData, FileMetadata, FileOsMetadata} from "./api-types/files";
import {invoke} from "@tauri-apps/api/tauri";
import {ApiFunction} from "./api-types/functions";
import {
    AddLocalFileREquest,
    AddRepositoryRequest,
    ChangeFileTagsRequest,
    CheckDaemonRunningRequest,
    CheckLocalRepositoryExistsRequest,
    CreateTagsRequest,
    DeleteRepositoryRequest,
    DeleteThumbnailsRequest,
    FindFilesRequest,
    GetFileMetadataRequest,
    GetSizeRequest,
    GetTagsForFilesRequest,
    InitRepositoryRequest,
    ReadFileRequest,
    RemoveRepositoryRequest,
    ResolvePathsToFilesRequest,
    RunJobRequest,
    SaveFileRequest,
    SelectRepositoryRequest,
    SetFrontendStateRequest,
    StartDaemonRequest,
    UpdateFileNameRequest
} from "./api-types/requests";
import {
    RepositoryData,
    RepositoryMetadata,
    SizeMetadata
} from "./api-types/repo";
import {NamespaceData, TagData} from "./api-types/tags";

export class MediarepoApi {

    public static async hasExecutable(): Promise<boolean> {
        return this.invokePlugin(ApiFunction.HasExecutable);
    }

    public static async getRepositories(): Promise<RepositoryData[]> {
        return this.invokePlugin(ApiFunction.GetRepositories);
    }

    public static async selectRepository(request: SelectRepositoryRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.SelectRepository, request);
    }

    public static async disconnectRepository(): Promise<void> {
        return this.invokePlugin(ApiFunction.DisconnectRepository);
    }

    public static async closeLocalRepository(): Promise<void> {
        return this.invokePlugin(ApiFunction.CloseLocalRepository);
    }

    public static async addRepository(request: AddRepositoryRequest): Promise<RepositoryData[]> {
        return this.invokePlugin(ApiFunction.AddRepository, request);
    }

    public static async checkDaemonRunning(request: CheckDaemonRunningRequest): Promise<boolean> {
        return this.invokePlugin(ApiFunction.CheckDaemonRunning, request);
    }

    public static async checkLocalRepositoryExists(request: CheckLocalRepositoryExistsRequest): Promise<boolean> {
        return this.invokePlugin(ApiFunction.CheckLocalRepositoryExists, request);
    }

    public static async removeRepository(request: RemoveRepositoryRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.RemoveRepository, request);
    }

    public static async deleteRepository(request: DeleteRepositoryRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.DeleteRepository, request);
    }

    public static async startDaemon(request: StartDaemonRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.StartDaemon, request);
    }

    public static async initRepository(request: InitRepositoryRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.InitRepository, request);
    }

    public static async getRepositoryMetadata(): Promise<RepositoryMetadata> {
        return this.invokePlugin(ApiFunction.GetRepoMetadata);
    }

    public static async getSize(request: GetSizeRequest): Promise<SizeMetadata> {
        return this.invokePlugin(ApiFunction.GetSize, request);
    }

    public static async getActiveRepository(): Promise<RepositoryData | undefined> {
        return this.invokePlugin(ApiFunction.GetActiveRepository);
    }

    public static async getAllFiles(): Promise<FileBasicData[]> {
        return this.invokePlugin(ApiFunction.GetAllFiles);
    }

    public static async findFiles(request: FindFilesRequest): Promise<FileBasicData[]> {
        return this.invokePlugin(ApiFunction.FindFiles, request);
    }

    public static async getFileMetadata(request: GetFileMetadataRequest): Promise<FileMetadata> {
        return this.invokePlugin(ApiFunction.GetFileMetadata, request);
    }

    public static async updateFileName(request: UpdateFileNameRequest): Promise<FileMetadata> {
        return this.invokePlugin(ApiFunction.UpdateFileName, request);
    }

    public static async saveFileLocally(request: SaveFileRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.SaveFileLocally, request);
    }

    public static async deleteThumbnails(request: DeleteThumbnailsRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.DeleteThumbnails, request);
    }

    public static async readFile(request: ReadFileRequest): Promise<number[]> {
        return this.invokePlugin(ApiFunction.ReadFile, request);
    }

    public static async getAllTags(): Promise<TagData[]> {
        return this.invokePlugin(ApiFunction.GetAllTags);
    }

    public static async getAllNamespaces(): Promise<NamespaceData[]> {
        return this.invokePlugin(ApiFunction.GetAllNamespace);
    }

    public static async getTagsForFiles(request: GetTagsForFilesRequest): Promise<TagData[]> {
        return this.invokePlugin(ApiFunction.GetTagsForFiles, request);
    }

    public static async createTags(request: CreateTagsRequest): Promise<TagData[]> {
        return this.invokePlugin(ApiFunction.CreateTags, request);
    }

    public static async changeFileTags(request: ChangeFileTagsRequest): Promise<TagData[]> {
        return this.invokePlugin(ApiFunction.ChangeFileTags, request);
    }

    public static async resolvePathsToFiles(request: ResolvePathsToFilesRequest): Promise<FileOsMetadata[]> {
        return this.invokePlugin(ApiFunction.ResolvePathsToFiles, request);
    }

    public static async addLocalFile(request: AddLocalFileREquest): Promise<FileBasicData> {
        return this.invokePlugin(ApiFunction.AddLocalFile, request);
    }

    public static async getFrontendState(): Promise<string> {
        return this.invokePlugin(ApiFunction.GetFrontendState);
    }

    public static async setFrontendState(request: SetFrontendStateRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.SetFrontendState, request);
    }

    public static async runJob(request: RunJobRequest): Promise<void> {
        return this.invokePlugin(ApiFunction.RunJob, request);
    }

    private static async invokePlugin<T>(fn: ApiFunction, args?: any): Promise<T> {
        return invoke<T>(`plugin:mediarepo|${fn}`, args);
    }
}
