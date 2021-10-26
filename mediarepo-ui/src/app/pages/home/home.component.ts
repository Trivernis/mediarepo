import {Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {FileService} from "../../services/file/file.service";
import {File} from "../../models/File";
import {Lightbox, LIGHTBOX_EVENT, LightboxEvent} from "ngx-lightbox";
import {ErrorBrokerService} from "../../services/error-broker/error-broker.service";
import {TagService} from "../../services/tag/tag.service";
import {Tag} from "../../models/Tag";
import {MatChipInputEvent} from "@angular/material/chips";
import {COMMA, ENTER} from "@angular/cdk/keycodes";
import {MatSelectionListChange} from "@angular/material/list";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {Observable} from "rxjs";
import {map, startWith} from "rxjs/operators";
import {FormControl} from "@angular/forms";
import {FileSearchComponent} from "../../components/file-search/file-search.component";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  tags: Tag[] = [];
  files: File[] = [];
  private openingLightbox = false;

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
  }

  async onFileMultiSelect(files: File[]) {
    if (files.length === 0) {
      this.clearFileDetails();
    }
  }

  async onFileSelect(file: File | undefined) {
    if (file) {
      await this.showFileDetails(file);
    } else {
      this.clearFileDetails();
    }
  }

  clearFileDetails() {
    this.tags = [];
  }

  async showFileDetails(file: File) {
    this.tags = await this.tagService.getTagsForFile(file.hash);
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
