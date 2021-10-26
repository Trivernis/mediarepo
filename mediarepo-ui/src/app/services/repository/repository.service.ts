import { Injectable } from '@angular/core';
import {Repository} from "../../models/Repository";
import {BehaviorSubject, Observable} from "rxjs";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import {Info} from "../../models/Info";
import {ErrorBrokerService} from "../error-broker/error-broker.service";
import {DataloaderService} from "../dataloader/dataloader.service";

@Injectable({
  providedIn: 'root'
})
export class RepositoryService {
  repositories = new BehaviorSubject<Repository[]>([]);
  public selectedRepository = new BehaviorSubject<Repository | undefined>(undefined);

  constructor(private errorBroker: ErrorBrokerService, private dataloaderService: DataloaderService) {
    this.registerListener()
  }

  async registerListener() {
    await listen("info", (event: { payload: Info }) => {
      const message = `Connected to ${event.payload.name}, Version: ${event.payload.version}`;
      this.errorBroker.showInfo(message);
    });
  }

  public async loadRepositories() {
    let active_repo = await invoke<Repository | undefined>("plugin:mediarepo|get_active_repository");
    this.selectedRepository.next(active_repo);

    let repos = await invoke<Repository[]>("plugin:mediarepo|get_repositories");
    this.repositories.next(repos);
  }

  public async setRepository(repo: Repository) {
    await invoke("plugin:mediarepo|select_repository", {name: repo.name});
    this.selectedRepository.next(repo);
    await this.dataloaderService.loadData();
  }

  public async addRepository(name: string, path: string) {
    let repos = await invoke<Repository[]>("plugin:mediarepo|add_repository", {name, path});
    this.repositories.next(repos);
  }
}
