import {Injectable} from "@angular/core";
import {Repository} from "../../../api/models/Repository";
import {BehaviorSubject} from "rxjs";
import {listen} from "@tauri-apps/api/event";
import {Info} from "../../models/Info";
import {ErrorBrokerService} from "../error-broker/error-broker.service";
import {RepositoryMetadata} from "../../models/RepositoryMetadata";
import {MediarepoApi} from "../../../api/Api";
import {mapMany, mapNew, mapOptional,} from "../../../api/models/adaptors";
import {SizeMetadata, SizeType} from "../../../api/api-types/repo";

@Injectable({
    providedIn: "root"
})
export class RepositoryService {
    repositories = new BehaviorSubject<Repository[]>([]);
    public selectedRepository = new BehaviorSubject<Repository | undefined>(
        undefined);

    constructor(private errorBroker: ErrorBrokerService) {
        this.registerListener().catch(err => console.error(err));
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
        return MediarepoApi.hasExecutable();
    }

    /**
     * Loads all repositories stored in the settings
     * @returns {Promise<void>}
     */
    public async loadRepositories() {
        await this.loadSelectedRepository();
        let repos = await MediarepoApi.getRepositories().then(mapMany(mapNew(Repository)));
        this.repositories.next(repos);
    }

    /**
     * Sets the active repository
     * @param {Repository} repo
     * @returns {Promise<void>}
     */
    public async setRepository(repo: Repository) {
        const selectedRepo = this.selectedRepository.getValue();
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
        await MediarepoApi.selectRepository({name: repo.name});
    }

    /**
     * Disconnects from a remote repository
     * @returns {Promise<void>}
     */
    public async disconnectSelectedRepository() {
        await MediarepoApi.disconnectRepository();
        await this.loadRepositories();
    }

    /**
     * Closes a local selected repository
     * @returns {Promise<void>}
     */
    public async closeSelectedRepository() {
        await MediarepoApi.closeLocalRepository();
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
        let repos = await MediarepoApi.addRepository({name, path, address, local}).then(mapMany(mapNew(Repository)));
        this.repositories.next(repos);
    }

    /**
     * Checks if a daemon is running for the specified address
     * @param {string} address
     * @returns {Promise<boolean>}
     */
    public async checkDaemonRunning(address: string): Promise<boolean> {
        return MediarepoApi.checkDaemonRunning({address});
    }

    /**
     * Checks if a local repository exists
     * @param {string} path
     * @returns {Promise<boolean>}
     */
    public async checkLocalRepositoryExists(path: string): Promise<boolean> {
        return await MediarepoApi.checkLocalRepositoryExists({path});
    }

    /**
     * Removes a repository from the list of saved repositories
     * @param {string} name
     * @returns {Promise<void>}
     */
    public async removeRepository(name: string): Promise<void> {
        await MediarepoApi.removeRepository({name});
        await this.loadRepositories();
    }

    /**
     * Deletes a local repository from the filesystem
     * @param {string} name
     * @returns {Promise<void>}
     */
    public async deleteRepository(name: string): Promise<void> {
        await MediarepoApi.deleteRepository({name});
        await this.removeRepository(name);
    }

    /**
     * Starts a daemon for the given repository path
     * @param {string} repoPath
     * @returns {Promise<void>}
     */
    public async startDaemon(repoPath: string): Promise<void> {
        return MediarepoApi.startDaemon({repoPath});
    }

    /**
     * Initializes a folder as a repository
     * @param {string} repoPath
     * @returns {Promise<void>}
     */
    public async initRepository(repoPath: string): Promise<void> {
        return MediarepoApi.initRepository({repoPath});
    }

    /**
     * Retrieves metadata about the selected repository
     * @returns {Promise<RepositoryMetadata>}
     */
    public async getRepositoryMetadata(): Promise<RepositoryMetadata> {
        return MediarepoApi.getRepositoryMetadata();
    }

    /**
     * Returns a specific size
     * @returns {Promise<SizeMetadata>}
     * @param sizeType
     */
    public async getSize(sizeType: SizeType): Promise<SizeMetadata> {
        return MediarepoApi.getSize({sizeType});
    }

    async loadSelectedRepository() {
        let active_repo = await MediarepoApi.getActiveRepository().then(mapOptional(mapNew(Repository)));
        this.selectedRepository.next(active_repo);
    }
}
