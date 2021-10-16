import { Component, OnInit } from '@angular/core';
import {FileService} from "../../services/file/file.service";
import {File} from "../../models/File";
import {PageEvent} from "@angular/material/paginator";
import {Lightbox, LIGHTBOX_EVENT, LightboxEvent} from "ngx-lightbox";
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  fileRows: File[][] = [];
  page: number = 0;
  pageSize: number = 25;

  constructor(private fileService: FileService, private lightbox: Lightbox, private lightboxEvent: LightboxEvent) { }

  async ngOnInit() {
    this.fileService.displayedFiles.subscribe((files) => this.setFileRows(files));
    await this.fileService.getFiles();
  }

  setFileRows(files: File[]) {
    this.fileRows = [];
    const filesPerRow = 6;
    for (let i = 0; i < (Math.ceil(files.length /filesPerRow )); i++) {
      this.fileRows.push(files.slice(i * filesPerRow, Math.min(files.length, (i + 1) * filesPerRow)))
    }
    console.log(this.fileRows);
  }

  async openFile(file: File) {
    let url = await this.fileService.readFile(file.hash, file.mime_type ?? "image/png");

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
    const lighboxSubscription = this.lightboxEvent.lightboxEvent$.subscribe((event: any) => {
      if (event?.id == LIGHTBOX_EVENT.CLOSE) {
        lighboxSubscription.unsubscribe();
        URL?.revokeObjectURL(url as string);
      }
    })
  }
}
