import {Component, OnInit, ViewChild} from '@angular/core';
import {Repository} from "../../models/Repository";
import {RepositoryService} from "../../services/repository/repository.service";
import {MatTabGroup} from "@angular/material/tabs";
import {TagService} from "../../services/tag/tag.service";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  public selectedRepository: Repository | undefined;

  @ViewChild("tabGroup") tabGroup!: MatTabGroup;


  constructor(private repoService: RepositoryService, private tagService: TagService) {
  }

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
}
