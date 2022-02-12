import {Injectable} from "@angular/core";
import {BehaviorSubject, Subscription} from "rxjs";
import {AppState} from "../../models/state/AppState";
import {FileService} from "../file/file.service";
import {RepositoryService} from "../repository/repository.service";
import {FilesTabState} from "../../models/state/FilesTabState";
import {debounceTime} from "rxjs/operators";
import {MediarepoApi} from "../../../api/Api";
import {StateServices} from "../../models/state/StateServices";
import {ImportTabState} from "../../models/state/ImportTabState";
import {TabState} from "../../models/state/TabState";
import {TabCategory} from "../../models/state/TabCategory";

@Injectable({
    providedIn: "root"
})
export class StateService {

    public state: BehaviorSubject<AppState>;

    private tabSubscriptions: Subscription[] = [];

    private stateChange = new BehaviorSubject<void>(undefined);

    constructor(private fileService: FileService, private repoService: RepositoryService) {
        this.state = new BehaviorSubject(new AppState(this.getServices()));
        this.repoService.selectedRepository.subscribe(async (repo) => {
            if (repo && (!this.state.value.repoName || this.state.value.repoName !== repo.name)) {
                await this.loadState();
            } else {
                const state = new AppState(this.getServices());
                this.subscribeToState(state);
                this.state.next(state);
            }
        });
        this.stateChange.pipe(debounceTime(1000))
            .subscribe(async () => this.saveState());
    }

    /**
     * Returns the state of the frontend
     * @returns {Promise<void>}
     */
    public async loadState() {
        let stateString = await MediarepoApi.getFrontendState();
        let state;

        if (stateString) {
            try {
                state = AppState.deserializeJson(stateString, this.getServices());
            } catch (err) {
                console.error("could not deserialize malformed state: ", err);
                state = new AppState(this.getServices());
            }
        } else {
            state = new AppState(this.getServices());
        }
        let selectedRepo = this.repoService.selectedRepository.value;
        if (selectedRepo) {
            state.repoName = selectedRepo.name;
        }
        this.subscribeToState(state);
        this.state.next(state);
    }

    /**
     * Sets the state of the frontend
     * @returns {Promise<void>}
     */
    public async saveState(): Promise<void> {
        if (this.repoService.selectedRepository.value) {
            await MediarepoApi.setFrontendState({ state: this.state.value.serializeJson() });
        }
    }

    private subscribeToState(state: AppState) {
        state.tabs.subscribe(async tabs => {
            this.tabSubscriptions.forEach(s => s.unsubscribe());
            tabs.forEach((tab) => this.subscribeToTab(tab));
            this.stateChange.next();
        });
        state.selectedTab.subscribe(() => this.stateChange.next());
    }

    private subscribeToTab(tab: TabState) {
        if (tab.category === TabCategory.Files) {
            this.subscribeToFilesTab(tab as FilesTabState);
        } else if (tab.category === TabCategory.Import) {
            this.subscribeToImportTab(tab as ImportTabState);
        }
    }

    private subscribeToImportTab(tab: ImportTabState) {
        this.tabSubscriptions.push(tab.mode
            .subscribe(() => this.stateChange.next()));
    }

    private subscribeToFilesTab(tab: FilesTabState) {
        this.tabSubscriptions.push(tab.filters
            .subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(tab.files
            .subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(tab.sortingPreset
            .subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(
            tab.selectedCD.subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(
            tab.mode.subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(tab.mode
            .subscribe(() => this.stateChange.next()));
    }

    private getServices(): StateServices {
        return new StateServices(this.fileService);
    }
}
