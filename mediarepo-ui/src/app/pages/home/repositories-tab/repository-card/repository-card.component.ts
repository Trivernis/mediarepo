import {Component, Input, OnInit} from '@angular/core';
import {Repository} from "../../../../models/Repository";
import {RepositoryService} from "../../../../services/repository/repository.service";
import {Router} from "@angular/router";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";

@Component({
  selector: 'app-repository-card',
  templateUrl: './repository-card.component.html',
  styleUrls: ['./repository-card.component.scss']
})
export class RepositoryCardComponent implements OnInit {

  @Input() repository!: Repository;

  public daemonRunning: boolean = false;

  constructor(private repoService: RepositoryService, private router: Router, private errorBroker: ErrorBrokerService) {}

  public async ngOnInit() {
    this.daemonRunning = await this.repoService.checkDaemonRunning(this.repository.address);
  }

  async startDaemonAndSelectRepository() {
    try {
      await this.repoService.startDaemon(this.repository.path!);
      this.daemonRunning = true;
      await new Promise((res, _) => {
        setTimeout(res, 2000) // wait for the daemon to start
      });
      await this.selectRepository();
    } catch (err) {
      this.errorBroker.showError(err);
    }
  }

  async selectRepository() {
      try {
        await this.repoService.setRepository(this.repository);
      } catch(err) {
        this.errorBroker.showError(err);
      }
  }
}
