import {Component, OnInit} from '@angular/core';
import {Router} from "@angular/router";
import {RepositoryService} from "./services/repository/repository.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {ErrorBrokerService} from "./services/error-broker/error-broker.service";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit{
  title = 'mediarepo-ui';

  constructor(
    private router: Router,
    private snackBar: MatSnackBar,
    private errorBroker: ErrorBrokerService,
    private repoService: RepositoryService,
  ) {
  }

  async ngOnInit() {
    this.errorBroker.errorCb = (err: { message: string }) => this.showError(err);
    this.errorBroker.infoCb = (info: string) => this.showInfo(info);
    await this.repoService.loadRepositories();
  }

  private showError(err: { message: string }) {
    this.snackBar.open(err.message, undefined, {
      panelClass: "warn",
      duration: 2000,
    });
  }

  private showInfo(info: string) {
    this.snackBar.open(info, undefined, {
      panelClass: "primary",
      duration: 2000,
    });
  }
}
