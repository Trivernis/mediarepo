import { Component } from '@angular/core';
import {Router} from "@angular/router";
import {RepositoryService} from "./services/repository/repository.service";
import {DataloaderService} from "./services/dataloader/dataloader.service";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'mediarepo-ui';

  constructor(
    private router: Router,
    private snackBar: MatSnackBar,
    private dataloaderService: DataloaderService,
    private repoService: RepositoryService
  ) {
  }

  async ngOnInit() {
    this.dataloaderService.loaderError.subscribe({
      error: (err) => {
        this.snackBar.open(err, undefined, {panelClass: "warn"})
      }
    })
    await this.dataloaderService.loadData();
    if (this.repoService.selectedRepository.getValue() == undefined) {
      await this.router.navigate(["repositories"])
    }
  }
}
