import { Injectable } from '@angular/core';
import {Repository} from "../../models/Repository";
import {BehaviorSubject, Observable} from "rxjs";
import {invoke} from "@tauri-apps/api/tauri";

@Injectable({
  providedIn: 'root'
})
export class RepositoryService {
  repositories = new BehaviorSubject<Repository[]>([]);
  public selectedRepository = new BehaviorSubject<Repository | undefined>(undefined);

  constructor() {}

  public async loadRepositories() {
    let repos = await invoke<Repository[]>("get_repositories");
    this.repositories.next(repos);
  }

  public setRepository(repo: Repository) {
    this.selectedRepository.next(repo);
  }

  public async addRepository(name: string, path: string) {
    let repos = await invoke<Repository[]>("add_repository", {name, path});
    this.repositories.next(repos);
  }
}
