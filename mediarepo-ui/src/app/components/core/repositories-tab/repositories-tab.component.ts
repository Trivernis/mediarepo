import {Component, OnInit} from '@angular/core';
import {Repository} from "../../../models/Repository";
import {RepositoryService} from "../../../services/repository/repository.service";
import {MatDialog} from "@angular/material/dialog";
import {AddRepositoryDialogComponent} from "./add-repository-dialog/add-repository-dialog.component";

@Component({
  selector: 'app-repositories-tab',
  templateUrl: './repositories-tab.component.html',
  styleUrls: ['./repositories-tab.component.scss']
})
export class RepositoriesTabComponent implements OnInit {

  repositories: Repository[] = [];

  constructor(
    private repoService: RepositoryService,
    public dialog: MatDialog
  ) {
  }

  ngOnInit(): void {
    this.repoService.repositories.subscribe({
      next: (repos) => {
        this.repositories = repos;
      }
    });
  }

  public openAddRepositoryDialog() {
    this.dialog.open(AddRepositoryDialogComponent, {
      disableClose: true,
      minWidth: "30%",
      minHeight: "30%",
    });
  }
}
