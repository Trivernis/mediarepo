import { Component, OnInit } from '@angular/core';
import {FileService} from "../../services/file/file.service";
import {File} from "../../models/File";
import {PageEvent} from "@angular/material/paginator";
import {Lightbox, LIGHTBOX_EVENT, LightboxEvent} from "ngx-lightbox";
import {SafeResourceUrl} from "@angular/platform-browser";
import {ErrorBrokerService} from "../../services/error-broker/error-broker.service";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  files: File[] = [];
  private openingLightbox = false;

  constructor(private errorBroker: ErrorBrokerService, private fileService: FileService, private lightbox: Lightbox, private lightboxEvent: LightboxEvent) { }

  async ngOnInit() {
    this.fileService.displayedFiles.subscribe((files) => this.files = files);
    await this.fileService.getFiles();
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
