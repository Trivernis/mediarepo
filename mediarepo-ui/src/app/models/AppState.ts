import {TabState} from "./TabState";
import {FileService} from "../services/file/file.service";
import {BehaviorSubject} from "rxjs";
import {TabCategory} from "./TabCategory";

export class AppState {

    private tabIdCounter = 0;
    public tabs = new BehaviorSubject<TabState[]>([]);
    public selectedTab = new BehaviorSubject<number | undefined>(undefined);

    private readonly fileService: FileService

    constructor(fileService: FileService) {
        this.fileService = fileService;
    }

    public addTab(category: TabCategory): TabState {
        const state = new TabState(this.tabIdCounter++, category, this.fileService);
        this.tabs.next([...this.tabs.value, state]);
        return state;
    }

    public async closeTab(uuid: number) {
        const index = this.tabs.value.findIndex(t => t.uuid === uuid);
        const tabs = this.tabs.value;
        tabs.splice(index, 1);
        this.tabs.next(tabs);
    }

    public static deserializeJson(stateString: string, fileService: FileService): AppState {
        let state = JSON.parse(stateString);
        let appState = new AppState(fileService);
        const tabs = state.tabs.map((tab: any) => TabState.fromDTO(tab, fileService));
        appState.tabs.next(tabs);

        appState.tabIdCounter = state.tabIdCounter;
        appState.selectedTab.next(state.selectedTab);

        return appState;
    }

    public serializeJson(): string {
        const tabDTOs = this.tabs.value.map(tab => tab.getDTO());
        return JSON.stringify({
            tabs: tabDTOs,
            tabIdCounter: this.tabIdCounter,
            selectedTab: this.selectedTab.value,
        });
    }
}
