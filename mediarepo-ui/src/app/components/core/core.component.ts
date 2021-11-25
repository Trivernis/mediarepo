import {Component, OnInit, ViewChild} from '@angular/core';
import {Repository} from "../../models/Repository";
import {RepositoryService} from "../../services/repository/repository.service";
import {MatTabChangeEvent, MatTabGroup} from "@angular/material/tabs";
import {TagService} from "../../services/tag/tag.service";
import {TabService} from "../../services/tab/tab.service";

@Component({
  selector: 'app-core',
  templateUrl: './core.component.html',
  styleUrls: ['./core.component.scss']
})
export class CoreComponent implements OnInit {

  public selectedRepository: Repository | undefined;

  @ViewChild("tabGroup") tabGroup!: MatTabGroup;


  constructor(
    private tabService: TabService,
    private repoService: RepositoryService,
    private tagService: TagService)
  {}

  public async ngOnInit(): Promise<void> {
    this.selectedRepository = this.repoService.selectedRepository.getValue();
    this.repoService.selectedRepository.subscribe(async (selected) => {
      this.selectedRepository = selected;
      this.updateSelectedTab();
      await this.loadRepoData();
    });
  }

  public updateSelectedTab() {
    if (!this.tabGroup) {
      return;
    }
    if (!this.selectedRepository) {
      this.tabGroup.selectedIndex = 0;
    } else if (this.tabGroup.selectedIndex === 0) {
      this.tabGroup.selectedIndex = 1;
    }
  }

  async loadRepoData() {
    await this.tagService.loadTags();
  }

  public onTabSelectionChange(event: MatTabChangeEvent): void {
    this.tabService.setSelectedTab(event.index);
  }
}
