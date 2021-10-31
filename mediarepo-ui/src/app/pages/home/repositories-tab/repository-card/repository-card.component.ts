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
export class RepositoryCardComponent {

  @Input() repository?: Repository;

  constructor(private repoService: RepositoryService, private router: Router, private errorBroker: ErrorBrokerService) {}

  async selectRepository() {
    if (this.repository) {
      try {
        await this.repoService.setRepository(this.repository);
      } catch(err) {
        this.errorBroker.showError(err);
      }
    }
  }
}
