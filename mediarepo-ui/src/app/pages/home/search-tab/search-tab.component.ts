import {Component, OnInit, ViewChild} from '@angular/core';
import {Tag} from "../../../models/Tag";
import {File} from "../../../models/File";
import {FileSearchComponent} from "../../../components/file-search/file-search.component";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";
import {FileService} from "../../../services/file/file.service";
import {TagService} from "../../../services/tag/tag.service";
import {Lightbox, LIGHTBOX_EVENT, LightboxEvent} from "ngx-lightbox";
import {MatSelectionListChange} from "@angular/material/list";
import {SortKey} from "../../../models/SortKey";
import {RepositoryService} from "../../../services/repository/repository.service";

@Component({
  selector: 'app-search-tab',
  templateUrl: './search-tab.component.html',
  styleUrls: ['./search-tab.component.scss']
})
export class SearchTabComponent implements OnInit {

  tags: Tag[] = [];
  files: File[] = [];
  private openingLightbox = false;
  showGallery = false;
  preselectedFile: File | undefined;
  contentLoading = false;

  @ViewChild('filesearch') fileSearch!: FileSearchComponent;

  constructor(
    private errorBroker: ErrorBrokerService,
    private repoService: RepositoryService,
    private fileService: FileService,
    private tagService: TagService,) {
  }

  async ngOnInit() {
    this.fileService.displayedFiles.subscribe((files) => this.files = files);
    this.repoService.selectedRepository.subscribe(async (repo) => repo && await this.loadFilesInitially());
    await this.loadFilesInitially();
  }

  async loadFilesInitially() {
    this.files = [];
    this.contentLoading = true;

    if (this.fileSearch) {
      await this.fileSearch.searchForFiles();
    } else {
      await this.fileService.findFiles([], [new SortKey("FileImportedTime", "Ascending", undefined)])
    }
    this.contentLoading = false;
  }

  async onFileMultiSelect(files: File[]) {
    await this.showFileDetails(files);
  }

  async onFileSelect(file: File | undefined) {
    if (file) {
      await this.showFileDetails([file]);
    } else {
      this.tags = [];
    }
  }

  async showFileDetails(files: File[]) {
    this.tags = [];

    for (const file of files) {
      const fileTags = await this.tagService.getTagsForFile(file.hash)
      for (const tag of fileTags) {
        if (this.tags.findIndex((t) => t.getNormalizedOutput() === tag.getNormalizedOutput()) < 0) {
          this.tags.push(tag);
        }
      }
    }

    this.tags = this.tags.sort((a, b) => {
      const aNorm = a.getNormalizedOutput();
      const bNorm = b.getNormalizedOutput();
      if (aNorm > bNorm) {
        return 1
      } else if (bNorm > aNorm) {
        return -1;
      } else {
        return 0;
      }
    });
  }

  async addSearchTagFromList(event: MatSelectionListChange) {
    if (event.options.length > 0) {
      const tag = event.options[0].value;
      this.fileSearch.addSearchTag(tag);
      await this.fileSearch.searchForFiles();
    }
    event.source.deselectAll();
  }

  async openGallery(preselectedFile: File) {
    this.preselectedFile = preselectedFile;
    this.showGallery = true;
  }

  async closeGallery(preselectedFile: File | undefined) {
    this.preselectedFile = preselectedFile;
    this.showGallery = false;
  }
}
