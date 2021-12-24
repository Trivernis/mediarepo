import {Injectable} from "@angular/core";
import {BehaviorSubject, Subscription} from "rxjs";
import {AppState} from "../../models/AppState";
import {invoke} from "@tauri-apps/api/tauri";
import {FileService} from "../file/file.service";
import {RepositoryService} from "../repository/repository.service";
import {TabState} from "../../models/TabState.rs";
import {debounceTime} from "rxjs/operators";

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
            if (repo) {
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
        let stateString = await invoke<string | undefined>(
            "plugin:mediarepo|get_frontend_state");
        let state;

        if (stateString) {
            state = AppState.deserializeJson(stateString, this.fileService)
        } else {
            state = new AppState(this.fileService);
        }
        this.subscribeToState(state);
        this.state.next(state);
    }

    private subscribeToState(state: AppState) {
        state.tabs.subscribe(async tabs => {
            this.tabSubscriptions.forEach(s => s.unsubscribe());
            tabs.forEach((tab) => this.subscribeToTab(tab));
            this.stateChange.next();
        })
    }

    private subscribeToTab(tab: TabState) {
        this.tabSubscriptions.push(tab.filters
            .subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(tab.sortKeys
            .subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(
            tab.selectedFileHash.subscribe(() => this.stateChange.next()));
        this.tabSubscriptions.push(
            tab.mode.subscribe(() => this.stateChange.next()))
    }

    /**
     * Sets the state of the frontend
     * @returns {Promise<void>}
     */
    public async saveState(): Promise<void> {
        await invoke("plugin:mediarepo|set_frontend_state",
            {state: this.state.value.serializeJson()})
    }
}
