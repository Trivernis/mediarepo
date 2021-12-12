import {Component, ViewChild} from "@angular/core";
import {Repository} from "../../models/Repository";
import {RepositoryService} from "../../services/repository/repository.service";
import {MatTabChangeEvent, MatTabGroup} from "@angular/material/tabs";
import {TagService} from "../../services/tag/tag.service";
import {TabService} from "../../services/tab/tab.service";
import {TabCategory} from "../../models/TabCategory";
import {TabState} from "../../models/TabState.rs";
import {AppState} from "../../models/AppState";
import {StateService} from "../../services/state/state.service";

@Component({
    selector: "app-core",
    templateUrl: "./core.component.html",
    styleUrls: ["./core.component.scss"]
})
export class CoreComponent {

    public selectedRepository: Repository | undefined;
    public tabs: TabState[] = [];
    public appState: AppState;
    public newTab = false;

    @ViewChild("tabGroup") tabGroup!: MatTabGroup;

    constructor(
        private tabService: TabService,
        private repoService: RepositoryService,
        private stateService: StateService,
        private tagService: TagService) {
        this.selectedRepository = this.repoService.selectedRepository.getValue();

        this.repoService.selectedRepository.subscribe(async (selected) => {
            this.selectedRepository = selected;

            if (this.selectedRepository) {
                await this.loadRepoData();
            } else {
                this.newTab = false;
            }
        });
        this.appState = this.stateService.state.getValue();

        this.stateService.state.subscribe(state => {
            this.appState = state;
            console.log("new state", state);
            if (this.appState.tabs.value.length === 0) {
                this.addTab();
            }
            state.tabs.subscribe(tabs => {
                console.log("new tabs", tabs);
                this.tabs = tabs;
                console.log(tabs);
                if (this.tabs.length === 0) {
                    this.addTab();
                }
            });
        })
    }

    async loadRepoData() {
        await this.tagService.loadTags();
    }

    public onTabSelectionChange(event: MatTabChangeEvent): void {
        this.tabService.setSelectedTab(event.index);
    }

    public addFilesTab(): void {
        this.appState.addTab(TabCategory.Files);
        this.tabGroup.selectedIndex = this.tabs.length;
        this.newTab = false;
    }

    public addImportTab(): void {
        this.appState.addTab(TabCategory.Import);
        this.tabGroup.selectedIndex = this.tabs.length;
        this.newTab = false;
    }

    public addTab(): void {
        if (this.tabGroup) {
            this.newTab = true;
            this.tabGroup.selectedIndex = this.tabs.length + 1;
        }
    }

    public async closeTab(tab: TabState) {
        const previousIndex = this.tabGroup.selectedIndex;
        await this.appState.closeTab(tab.uuid);

        if (previousIndex) {
            if (previousIndex === 1 && this.tabs.length >= 1) {
                this.tabGroup.selectedIndex = previousIndex;
            } else {
                this.tabGroup.selectedIndex = previousIndex - 1;
            }
        } else {
            this.tabGroup.selectedIndex = 0;
        }
    }

    public async onMouseClickTabLabel(tab: TabState, event: MouseEvent) {
        console.log(event);
        if (event.button === 1) { // middle mouse button
            await this.closeTab(tab);
        }
    }
}
