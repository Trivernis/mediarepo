import {Injectable} from "@angular/core";
import {BehaviorSubject, Subscription} from "rxjs";
import {AppState} from "../../models/AppState";
import {FileService} from "../file/file.service";
import {RepositoryService} from "../repository/repository.service";
import {TabState} from "../../models/TabState";
import {debounceTime} from "rxjs/operators";
import {MediarepoApi} from "../../../api/Api";

@Injectable({
    providedIn: "root"
})
export class StateService {

    public state: BehaviorSubject<AppState>;

    private tabSubscriptions: Subscription[] = [];

    private stateChange = new BehaviorSubject<void>(undefined);

    constructor(private fileService: FileService, private repoService: RepositoryService) {
        this.state = new BehaviorSubject(new AppState(fileService));
        this.repoService.selectedRepository.subscribe(async (repo) => {
            if (repo && (!this.state.value.repoName || this.state.value.repoName !== repo.name)) {
                await this.loadState();
            } else {
                const state = new AppState(this.fileService);
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
            state = AppState.deserializeJson(stateString, this.fileService);
        } else {
            state = new AppState(this.fileService);
        }
        let selectedRepo = this.repoService.selectedRepository.value;
        if (selectedRepo) {
            state.repoName = selectedRepo.name;
        }
        this.subscribeToState(state);
        this.state.next(state);
    }

    private subscribeToState(state: AppState) {
        state.tabs.subscribe(async tabs => {
            this.tabSubscriptions.forEach(s => s.unsubscribe());
            tabs.forEach((tab) => this.subscribeToTab(tab));
            this.stateChange.next();
        });
    }

    private subscribeToTab(tab: TabState) {
        this.tabSubscriptions.push(tab.filters
            .subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(tab.sortKeys
            .subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(
            tab.selectedCD.subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(
            tab.mode.subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(tab.files.subscribe(() => this.stateChange.next()));
    }

    /**
     * Sets the state of the frontend
     * @returns {Promise<void>}
     */
    public async saveState(): Promise<void> {
        if (this.repoService.selectedRepository.value) {
            await MediarepoApi.setFrontendState({state: this.state.value.serializeJson()});
        }
    }
}
