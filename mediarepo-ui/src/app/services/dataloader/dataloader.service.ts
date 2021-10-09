import { Injectable } from '@angular/core';
import {RepositoryService} from "../repository/repository.service";
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class DataloaderService {

  loaderError = new BehaviorSubject(undefined);

  constructor(private repositoryService: RepositoryService) { }

  public async loadData() {
    try {
      await this.repositoryService.loadRepositories();
    } catch (err) {
      this.loaderError.error(err);
      console.error(err);
    }
  }
}
