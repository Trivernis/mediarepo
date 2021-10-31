import {Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {DataloaderService} from "../../services/dataloader/dataloader.service";
import {Repository} from "../../models/Repository";
import {RepositoryService} from "../../services/repository/repository.service";
import {MatTabGroup} from "@angular/material/tabs";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  public selectedRepository: Repository | undefined;

  @ViewChild("tabGroup") tabGroup!: MatTabGroup;

  public async ngOnInit(): Promise<void> {
    await this.dataloaderService.loadData();
    this.selectedRepository = this.repoService.selectedRepository.getValue();
    this.repoService.selectedRepository.subscribe((selected) => {
      this.selectedRepository = selected;
      this.updateSelectedTab();
    });
  }

  public updateSelectedTab() {
    if (!this.selectedRepository) {
      this.tabGroup.selectedIndex = 0;
    } else if (this.tabGroup.selectedIndex === 0) {
      this.tabGroup.selectedIndex = 1;
    }
  }

  constructor(private dataloaderService: DataloaderService, private repoService: RepositoryService) {}
}
