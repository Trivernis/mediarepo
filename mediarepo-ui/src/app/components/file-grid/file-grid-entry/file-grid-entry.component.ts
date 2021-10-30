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
import {GridEntry} from "./GridEntry";

@Component({
  selector: 'app-file-grid-entry',
  templateUrl: './file-grid-entry.component.html',
  styleUrls: ['./file-grid-entry.component.scss']
})
export class FileGridEntryComponent implements OnInit, OnDestroy {

  @ViewChild("card") card!: ElementRef;
  @Input() public gridEntry!: GridEntry;
  @Output() clickEvent = new EventEmitter<FileGridEntryComponent>();
  @Output() dblClickEvent = new EventEmitter<FileGridEntryComponent>();
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
    }
  }

  async loadImage() {
    try {
        const thumbnails = await this.fileService.getThumbnails(this.gridEntry.file.hash);
        let thumbnail = thumbnails.find(t => (t.height > 250 || t.width > 250) && (t.height < 500 && t.width < 500));
        this.selectedThumbnail = thumbnail ?? thumbnails[0];

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
