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

@Component({
  selector: 'app-search-page',
  templateUrl: './search-page.component.html',
  styleUrls: ['./search-page.component.scss']
})
export class SearchPageComponent implements OnInit {

  tags: Tag[] = [];
  files: File[] = [];
  private openingLightbox = false;
  showGallery = false;
  preselectedFile: File | undefined;
  contentLoading = false;

  @ViewChild('filesearch') fileSearch!: FileSearchComponent;

  constructor(
    private errorBroker: ErrorBrokerService,
    private fileService: FileService,
    private tagService: TagService,
    private lightbox: Lightbox,
    private lightboxEvent: LightboxEvent) {
  }

  async ngOnInit() {
    this.fileService.displayedFiles.subscribe((files) => this.files = files);
    this.contentLoading = true;
    await this.fileService.findFiles([], [new SortKey("FileImportedTime", "Ascending", undefined)])
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

  async openFile(file: File) {
    if (this.openingLightbox) {
      return;
    }
    this.openingLightbox = true;
    try {
      await this.openLightbox(file);
    } catch (err) {
      this.errorBroker.showError(err);
    }
    this.openingLightbox = false;
  }

  async openGallery(preselectedFile: File) {
    this.preselectedFile = preselectedFile;
    this.showGallery = true;
  }

  async closeGallery(preselectedFile: File | undefined) {
    this.preselectedFile = preselectedFile;
    this.showGallery = false;
  }

  private async openLightbox(file: File): Promise<void> {
    let url = await this.fileService.readFile(file);

    let albums = [
      {
        src: url as string,
        caption: file.name ?? file.comment,
        thumb: url as string,
      }
    ];
    this.lightbox.open(albums, 0, {
      disableScrolling: true,
      showImageNumberLabel: false,
      showDownloadButton: true,
      centerVertically: true,
    });
    const lighboxSubscription = this.lightboxEvent.lightboxEvent$.subscribe(
      (event: any) => {
        if (event?.id == LIGHTBOX_EVENT.CLOSE) {
          lighboxSubscription.unsubscribe();
          URL?.revokeObjectURL(url as string);
        }
      })
  }
}
