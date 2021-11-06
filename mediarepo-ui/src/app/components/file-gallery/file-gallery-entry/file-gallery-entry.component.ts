import {
  Component,
  EventEmitter,
  Input,
  OnChanges,
  OnInit,
  Output,
  SimpleChanges
} from '@angular/core';
import {File} from "../../../models/File";
import {FileService} from "../../../services/file/file.service";
import {SafeResourceUrl} from "@angular/platform-browser";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";
import {Selectable} from "../../../models/Selectable";

@Component({
  selector: 'app-file-gallery-entry',
  templateUrl: './file-gallery-entry.component.html',
  styleUrls: ['./file-gallery-entry.component.scss']
})
export class FileGalleryEntryComponent implements OnInit, OnChanges {

  @Input() file!: Selectable<File>;
  @Output() fileSelectEvent = new EventEmitter<Selectable<File>>();
  contentUrl: SafeResourceUrl | undefined;

  private cachedFile: File | undefined;

  constructor(private fileService: FileService, private errorBroker: ErrorBrokerService) {
  }

  async ngOnChanges(changes: SimpleChanges): Promise<void> {
    if (changes["file"] && (!this.cachedFile || this.file.data.hash !== this.cachedFile!.hash)) { // handle changes to the file when the component is not destroyed
      this.cachedFile = this.file.data;
      this.contentUrl = undefined;
      await this.loadImage();
    } else if (!this.contentUrl) {
      await this.loadImage();
    }
  }

  async ngOnInit() {
    this.cachedFile = this.file.data;
    await this.loadImage();
  }

  async loadImage() {
    try {
      const hash = this.file.data.hash;
      const thumbnails = await this.fileService.getThumbnails(this.file.data);
      let thumbnail = thumbnails.find(
        t => (t.height > 250 || t.width > 250) && (t.height < 500 && t.width < 500));
      thumbnail = thumbnail ?? thumbnails[0];

      if (!thumbnail) {
        console.log("Thumbnail is empty?!", thumbnails);
      } else if (this.file.data.hash === hash) {
        this.contentUrl = await this.fileService.readThumbnail(thumbnail!!);
      } else {
        console.warn("Grid file updated while loading thumbnail.")
      }
    } catch (err) {
      this.errorBroker.showError(err);
    }
  }
}
