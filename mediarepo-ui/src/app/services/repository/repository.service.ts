import { Injectable } from '@angular/core';
import {Repository} from "../../models/Repository";
import {BehaviorSubject, Observable} from "rxjs";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import {Info} from "../../models/Info";
import {ErrorBrokerService} from "../error-broker/error-broker.service";

@Injectable({
  providedIn: 'root'
})
export class RepositoryService {
  repositories = new BehaviorSubject<Repository[]>([]);
  public selectedRepository = new BehaviorSubject<Repository | undefined>(undefined);

  constructor(private errorBroker: ErrorBrokerService) {
    this.registerListener()
  }

  async registerListener() {
    await listen("info", (event: { payload: Info }) => {
      const message = `Connected to ${event.payload.name}, Version: ${event.payload.version}`;
      this.errorBroker.showInfo(message);
    });
  }

  public async loadRepositories() {
    let active_repo = await invoke<Repository | undefined>("get_active_repository");
    this.selectedRepository.next(active_repo);

    let repos = await invoke<Repository[]>("get_repositories");
    this.repositories.next(repos);
  }

  public async setRepository(repo: Repository) {
    await invoke("select_repository", {name: repo.name});
    await invoke("emit_info");
    this.selectedRepository.next(repo);
  }

  public async addRepository(name: string, path: string) {
    let repos = await invoke<Repository[]>("add_repository", {name, path});
    this.repositories.next(repos);
  }
}
