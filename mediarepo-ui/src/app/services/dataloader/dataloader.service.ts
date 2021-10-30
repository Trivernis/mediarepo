import { Injectable } from '@angular/core';
import {RepositoryService} from "../repository/repository.service";
import {BehaviorSubject} from "rxjs";
import {ErrorBrokerService} from "../error-broker/error-broker.service";
import {TagService} from "../tag/tag.service";
import {FileService} from "../file/file.service";

@Injectable({
  providedIn: 'root'
})
export class DataloaderService {

  constructor(
    private erroBroker: ErrorBrokerService,
    private fileService: FileService,
    private tagService: TagService) { }

  public async loadData() {
    try {
      await this.tagService.loadTags();
      await this.fileService.findFiles([]);
    } catch (err) {
      this.erroBroker.showError(err);
    }
  }
}
