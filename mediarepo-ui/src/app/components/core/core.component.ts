import {Component, OnInit, ViewChild} from "@angular/core";
import {Repository} from "../../models/Repository";
import {RepositoryService} from "../../services/repository/repository.service";
import {MatTabChangeEvent, MatTabGroup} from "@angular/material/tabs";
import {TagService} from "../../services/tag/tag.service";
import {TabService} from "../../services/tab/tab.service";
import {TabCategory} from "../../models/TabCategory";
import {TabState} from "../../models/TabState.rs";

@Component({
    selector: "app-core",
    templateUrl: "./core.component.html",
    styleUrls: ["./core.component.scss"]
})
export class CoreComponent implements OnInit {

    public selectedRepository: Repository | undefined;
    public tabs: TabState[] = [];

    @ViewChild("tabGroup") tabGroup!: MatTabGroup;
    public newTab = false;

    constructor(
        private tabService: TabService,
        private repoService: RepositoryService,
        private tagService: TagService) {
    }

    public async ngOnInit(): Promise<void> {
        this.selectedRepository = this.repoService.selectedRepository.getValue();
        this.repoService.selectedRepository.subscribe(async (selected) => {
            this.selectedRepository = selected;

            if (this.selectedRepository) {
                await this.loadRepoData();
                this.addTab();
            } else {
                this.newTab = false;
                this.tabService.closeAllTabs();
            }
        });
        this.tabService.tabs.subscribe(tabs => {
            this.tabs = tabs;
        });
    }

    async loadRepoData() {
        await this.tagService.loadTags();
    }

    public onTabSelectionChange(event: MatTabChangeEvent): void {
        this.tabService.setSelectedTab(event.index);
    }

    public addFilesTab(): void {
        this.tabService.addTab(TabCategory.Files);
        this.tabGroup.selectedIndex = this.tabs.length;
        this.newTab = false;
    }

    public addImportTab(): void {
        this.tabService.addTab(TabCategory.Import);
        this.tabGroup.selectedIndex = this.tabs.length;
        this.newTab = false;
    }

    public addTab(): void {
        this.newTab = true;
        this.tabGroup.selectedIndex = this.tabs.length + 1;
    }

    public closeTab(tab: TabState): void {
        const previousIndex = this.tabGroup.selectedIndex;
        this.tabService.closeTab(tab.uuid);

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

    public onMouseClickTabLabel(tab: TabState, event: MouseEvent): void {
        console.log(event);
        if (event.button === 1) { // middle mouse button
            this.closeTab(tab);
        }
    }
}
