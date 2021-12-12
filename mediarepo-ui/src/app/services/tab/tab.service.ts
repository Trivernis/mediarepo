import {Injectable} from "@angular/core";
import {BehaviorSubject} from "rxjs";
import {TabState} from "../../models/TabState.rs";
import {TabCategory} from "../../models/TabCategory";
import {FileService} from "../file/file.service";

@Injectable({
    providedIn: "root"
})
export class TabService {

    private tabIdCounter = 0;
    public selectedTab = new BehaviorSubject<number>(0);
    public tabs = new BehaviorSubject<TabState[]>([]);

    constructor(private fileService: FileService) {
    }

    public setSelectedTab(index: number) {
        this.selectedTab.next(index);
    }

    public addTab(category: TabCategory): TabState {
        const state = new TabState(this.tabIdCounter++, category, this.fileService);
        this.tabs.next([...this.tabs.value, state]);
        return state;
    }

    public closeTab(uuid: number) {
        const index = this.tabs.value.findIndex(t => t.uuid === uuid);
        const tabs = this.tabs.value;
        tabs.splice(index, 1)
        this.tabs.next(tabs);
    }

    public closeAllTabs() {
        this.tabs.next([]);
    }
}
