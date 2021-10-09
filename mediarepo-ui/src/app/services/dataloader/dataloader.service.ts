import { Injectable } from '@angular/core';
import {RepositoryService} from "../repository/repository.service";
import {BehaviorSubject} from "rxjs";
import {ErrorBrokerService} from "../error-broker/error-broker.service";

@Injectable({
  providedIn: 'root'
})
export class DataloaderService {

  constructor(private erroBroker: ErrorBrokerService, private repositoryService: RepositoryService) { }

  public async loadData() {
    try {
      await this.repositoryService.loadRepositories();
    } catch (err) {
      this.erroBroker.showError(err);
    }
  }
}
