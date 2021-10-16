import {
  Component,
  Input,
  OnInit,
  ViewChild,
  ElementRef, Output, EventEmitter, OnDestroy
} from '@angular/core';
import {File} from "../../../models/File";
import {FileService} from "../../../services/file/file.service";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";
import {SafeResourceUrl} from "@angular/platform-browser";
import {MatCard} from "@angular/material/card";
import {Thumbnail} from "../../../models/Thumbnail";

@Component({
  selector: 'app-file-grid-entry',
  templateUrl: './file-grid-entry.component.html',
  styleUrls: ['./file-grid-entry.component.scss']
})
export class FileGridEntryComponent implements OnInit, OnDestroy {

  @ViewChild("card") card!: ElementRef;
  @Input() file!: File;
  @Output() clickEvent = new EventEmitter<File>();
  @Output() dblClickEvent = new EventEmitter<File>();
  selectedThumbnail: Thumbnail | undefined;

  contentUrl: SafeResourceUrl | undefined;
  constructor(private fileService: FileService, private errorBroker: ErrorBrokerService) { }

  async ngOnInit() {
    await this.loadImage();
  }

  public ngOnDestroy(): void {
    if (this.contentUrl) {
      const url = this.contentUrl;
      this.contentUrl = undefined;
      URL?.revokeObjectURL(url as string);
    }
  }

  async loadImage() {
    try {
      const thumbnails = await this.fileService.getThumbnails(this.file.hash);
      let thumbnail = thumbnails.find(t => (t.height > 250 || t.width > 250) && (t.height < 500 && t.width < 500));
      this.selectedThumbnail = thumbnail;
      if (!thumbnail) {
        console.log("Thumbnail is empty?!", thumbnails);
      } else {
        this.contentUrl = await this.fileService.readThumbnail(thumbnail!!);
      }
    } catch (err) {
      this.errorBroker.showError(err);
    }
  }
}
