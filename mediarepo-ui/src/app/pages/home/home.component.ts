import { Component, OnInit } from '@angular/core';
import {FileService} from "../../services/file/file.service";
import {File} from "../../models/File";
import {PageEvent} from "@angular/material/paginator";
import {Lightbox, LIGHTBOX_EVENT, LightboxEvent} from "ngx-lightbox";
import {SafeResourceUrl} from "@angular/platform-browser";
import {ErrorBrokerService} from "../../services/error-broker/error-broker.service";
import {TagService} from "../../services/tag/tag.service";
import {Tag} from "../../models/Tag";
import {MatChipInputEvent} from "@angular/material/chips";
import {COMMA, ENTER} from "@angular/cdk/keycodes";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  files: File[] = [];
  tags: Tag[] = [];
  searchTags: string[] = [];
  private openingLightbox = false;
  searchInputSeparators = [ENTER, COMMA];

  constructor(
    private errorBroker: ErrorBrokerService,
    private fileService: FileService,
    private tagService: TagService,
    private lightbox: Lightbox,
    private lightboxEvent: LightboxEvent) { }

  async ngOnInit() {
    this.fileService.displayedFiles.subscribe((files) => this.files = files);
    await this.fileService.getFiles();
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

  async removeSearchTag(tag: string) {
    const index = this.searchTags.indexOf(tag);
    if (index >= 0) {
      this.searchTags.splice(index, 1);
    }
    await this.fileService.findFiles(this.searchTags);
  }

  async addSearchTag(event: MatChipInputEvent) {
    const tag = event.value.trim();
    if (tag.length > 0) {
      this.searchTags.push(tag);
      event.chipInput?.clear();
      await this.fileService.findFiles(this.searchTags);
    }
  }

  async openFile(file: File) {
    if (this.openingLightbox) {
      return;
    }
    this.openingLightbox = true;
    try {
      await this.openLightbox(file);
    } catch(err) {
      this.errorBroker.showError(err);
    }
    this.openingLightbox = false;
  }

  private async openLightbox(file: File): Promise<void> {
    let url = await this.fileService.readFile(file.hash,
      file.mime_type ?? "image/png");

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
