import {TabState} from "./TabState.rs";
import {FileService} from "../services/file/file.service";

export class AppState {

    public tabs: TabState[] = [];

    constructor() {
    }

    public static deserializeJson(stateString: string, fileService: FileService): AppState {
        let state = JSON.parse(stateString);
        let appState = new AppState();
        for (let tab of state.tabs) {
            appState.tabs.push(TabState.fromDTO(tab, fileService));
        }

        return appState
    }

    public serializeJson(): string {
        const tabDTOs = this.tabs.map(tab => tab.getDTO());
        return JSON.stringify({
            tabs: tabDTOs
        });
    }
}
