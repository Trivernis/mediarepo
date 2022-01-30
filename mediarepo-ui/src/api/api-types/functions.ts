export enum ApiFunction {
    // repository
    HasExecutable = "has_executable",
    GetRepositories = "get_repositories",
    SelectRepository = "select_repository",
    DisconnectRepository = "disconnect_repository",
    CloseLocalRepository = "close_local_repository",
    AddRepository = "add_repository",
    CheckDaemonRunning = "check_daemon_running",
    CheckLocalRepositoryExists = "check_local_repository_exists",
    RemoveRepository = "remove_repository",
    DeleteRepository = "delete_repository",
    StartDaemon = "start_daemon",
    InitRepository = "init_repository",
    GetRepoMetadata = "get_repo_metadata",
    GetSize = "get_size",
    GetActiveRepository = "get_active_repository",
    // files
    GetAllFiles = "get_all_files",
    FindFiles = "find_files",
    GetFileMetadata = "get_file_metadata",
    UpdateFileName = "update_file_name",
    UpdateFileStatus = "update_file_status",
    SaveFileLocally = "save_file_locally",
    DeleteThumbnails = "delete_thumbnails",
    ReadFile = "read_file",
    DeleteFile = "delete_file",
    // tags
    GetAllTags = "get_all_tags",
    GetAllNamespace = "get_all_namespaces",
    GetTagsForFiles = "get_tags_for_files",
    GetFileTagMap = "get_file_tag_map",
    CreateTags = "create_tags",
    ChangeFileTags = "change_file_tags",
    // import
    ResolvePathsToFiles = "resolve_paths_to_files",
    AddLocalFile = "add_local_file",
    // state
    GetFrontendState = "get_frontend_state",
    SetFrontendState = "set_frontend_state",
    // jobs
    RunJob = "run_job",
}
