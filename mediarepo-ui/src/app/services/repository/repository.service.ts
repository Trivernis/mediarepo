import {Injectable} from "@angular/core";
import {Repository} from "../../models/Repository";
import {BehaviorSubject} from "rxjs";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import {Info} from "../../models/Info";
import {ErrorBrokerService} from "../error-broker/error-broker.service";
import {FileService} from "../file/file.service";
import {RepositoryMetadata} from "../../models/RepositoryMetadata";
import {SizeMetadata, SizeType} from "../../models/SizeMetadata";

@Injectable({
    providedIn: "root"
})
export class RepositoryService {
    repositories = new BehaviorSubject<Repository[]>([]);
    public selectedRepository = new BehaviorSubject<Repository | undefined>(
        undefined);

    constructor(private errorBroker: ErrorBrokerService, private fileService: FileService) {
        this.registerListener()
    }

    /// Registers the info listener
    async registerListener() {
        await listen("info", (event: { payload: Info }) => {
            const message = `Connected to ${event.payload.name}, Version: ${event.payload.version}`;
            this.errorBroker.showInfo(message);
        });
    }

    /**
     * Checks if a daemon is configured in the settings or can be found on the system
     * @returns {Promise<boolean>}
     */
    public async checkDameonConfigured(): Promise<boolean> {
        return await invoke<boolean>("plugin:mediarepo|has_executable");
    }

    /**
     * Loads all repositories stored in the settings
     * @returns {Promise<void>}
     */
    public async loadRepositories() {
        await this.loadSelectedRepository();
        let repos = await invoke<Repository[]>(
            "plugin:mediarepo|get_repositories");
        this.repositories.next(repos);
    }

    /**
     * Sets the active repository
     * @param {Repository} repo
     * @returns {Promise<void>}
     */
    public async setRepository(repo: Repository) {
        const selectedRepo = this.selectedRepository.getValue()
        if (selectedRepo) {
            if (selectedRepo.local) {
                await this.closeSelectedRepository();
            } else {
                await this.disconnectSelectedRepository();
            }
        } else {
            try {
                // just to make sure because sometimes there's some weird issues
                await this.disconnectSelectedRepository();
            } catch (err) {
                console.warn(err);
            }

        }
        await invoke("plugin:mediarepo|select_repository", {name: repo.name});
        await this.loadRepositories();
    }

    /**
     * Disconnects from a remote repository
     * @returns {Promise<void>}
     */
    public async disconnectSelectedRepository() {
        await invoke("plugin:mediarepo|disconnect_repository");
        await this.loadRepositories();
    }

    /**
     * Closes a local selected repository
     * @returns {Promise<void>}
     */
    public async closeSelectedRepository() {
        await invoke("plugin:mediarepo|close_local_repository");
        await this.loadRepositories();
    }

    /**
     * Adds a respository to the repository list in the settings
     * @param {string} name
     * @param {string} path
     * @param address
     * @param local
     * @returns {Promise<void>}
     */
    public async addRepository(name: string, path: string | undefined, address: string | undefined, local: boolean) {
        let repos = await invoke<Repository[]>(
            "plugin:mediarepo|add_repository",
            {name, path, address, local});
        this.repositories.next(repos);
    }

    /**
     * Checks if a daemon is running for the specified address
     * @param {string} address
     * @returns {Promise<boolean>}
     */
    public async checkDaemonRunning(address: string): Promise<boolean> {
        return await invoke<boolean>("plugin:mediarepo|check_daemon_running",
            {address});
    }

    /**
     * Checks if a local repository exists
     * @param {string} path
     * @returns {Promise<boolean>}
     */
    public async checkLocalRepositoryExists(path: string): Promise<boolean> {
        return await invoke<boolean>(
            "plugin:mediarepo|check_local_repository_exists", {path})
    }

    /**
     * Removes a repository from the list of saved repositories
     * @param {string} name
     * @returns {Promise<void>}
     */
    public async removeRepository(name: string): Promise<void> {
        await invoke("plugin:mediarepo|remove_repository", {name});
        await this.loadRepositories();
    }

    /**
     * Deletes a local repository from the filesystem
     * @param {string} name
     * @returns {Promise<void>}
     */
    public async deleteRepository(name: string): Promise<void> {
        await invoke("plugin:mediarepo|delete_repository", {name});
        await this.removeRepository(name);
    }

    /**
     * Starts a daemon for the given repository path
     * @param {string} repoPath
     * @returns {Promise<void>}
     */
    public async startDaemon(repoPath: string): Promise<void> {
        await invoke("plugin:mediarepo|start_daemon", {repoPath})
    }

    /**
     * Initializes a folder as a repository
     * @param {string} repoPath
     * @returns {Promise<void>}
     */
    public async initRepository(repoPath: string): Promise<void> {
        await invoke("plugin:mediarepo|init_repository", {repoPath});
    }

    /**
     * Retrieves metadata about the selected repository
     * @returns {Promise<RepositoryMetadata>}
     */
    public async getRepositoryMetadata(): Promise<RepositoryMetadata> {
        return await invoke<RepositoryMetadata>("plugin:mediarepo|get_repo_metadata");
    }

    /**
     * Returns a specific size
     * @param {SizeType} type
     * @returns {Promise<SizeMetadata>}
     */
    public async getSize(sizeType: SizeType): Promise<SizeMetadata> {
        return await invoke<SizeMetadata>("plugin:mediarepo|get_size", {sizeType});
    }

    async loadSelectedRepository() {
        let active_repo = await invoke<Repository | undefined>(
            "plugin:mediarepo|get_active_repository");
        this.selectedRepository.next(active_repo);
    }
}
