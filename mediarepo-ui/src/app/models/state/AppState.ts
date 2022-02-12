import {FilesTabSaveState, FilesTabState} from "./FilesTabState";
import {BehaviorSubject} from "rxjs";
import {TabCategory} from "./TabCategory";
import {TabSaveState, TabState} from "./TabState";
import {StateServices} from "./StateServices";
import {ImportTabSaveState, ImportTabState} from "./ImportTabState";

export class AppState {

    public tabs = new BehaviorSubject<TabState[]>([]);
    public selectedTab = new BehaviorSubject<number | undefined>(undefined);
    public repoName: string | undefined;
    private tabIdCounter = 0;
    private readonly services: StateServices;

    constructor(services: StateServices) {
        this.services = services;
    }

    public static deserializeJson(stateString: string, services: StateServices): AppState {
        let state = JSON.parse(stateString);
        let appState = new AppState(services);

        const tabs = state.tabs.map((saveState: TabSaveState) => {
            let tab;

            if (saveState.category === TabCategory.Files) {
                tab = new FilesTabState(saveState.uuid, services);
                tab.restoreSaveState(saveState as FilesTabSaveState);
            } else if (saveState.category === TabCategory.Import) {
                tab = new ImportTabState(saveState.uuid, services);
                tab.restoreSaveState(saveState as ImportTabSaveState);
            }

            return tab;
        });
        appState.tabs.next(tabs);

        appState.tabIdCounter = state.tabIdCounter;
        appState.selectedTab.next(state.selectedTab);
        appState.repoName = state.repoName;

        return appState;
    }

    public addTab(category: TabCategory): TabState {
        let state;

        if (category == TabCategory.Files) {
            state = new FilesTabState(this.tabIdCounter++, this.services);
        } else {
            state = new ImportTabState(this.tabIdCounter++, this.services);
        }

        this.tabs.next([...this.tabs.value, state]);
        return state;
    }

    public async closeTab(uuid: number) {
        const index = this.tabs.value.findIndex(t => t.uuid === uuid);
        const tabs = this.tabs.value;
        tabs.splice(index, 1);
        this.tabs.next(tabs);
    }

    public serializeJson(): string {
        const tabDTOs = this.tabs.value.map(tab => tab.toSaveState());
        return JSON.stringify({
            repoName: this.repoName,
            tabs: tabDTOs,
            tabIdCounter: this.tabIdCounter,
            selectedTab: this.selectedTab.value,
        });
    }
}
