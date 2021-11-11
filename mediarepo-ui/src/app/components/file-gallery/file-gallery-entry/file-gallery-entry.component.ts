import {
  Component,
  EventEmitter, Inject,
  Input,
  OnChanges,
  OnInit,
  Output,
  SimpleChanges
} from '@angular/core';
import {File} from "../../../models/File";
import {FileService} from "../../../services/file/file.service";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
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

  constructor(@Inject(DomSanitizer) private sanitizer: DomSanitizer, private fileService: FileService, private errorBroker: ErrorBrokerService) {
  }

  async ngOnChanges(changes: SimpleChanges): Promise<void> {
    if (changes["file"] && (!this.cachedFile || this.file.data.hash !== this.cachedFile!.hash)) { // handle changes to the file when the component is not destroyed
      this.cachedFile = this.file.data;
      this.contentUrl = this.fileService.buildThumbnailUrl(this.file.data, 250, 250);
    }
  }

  async ngOnInit() {
    this.cachedFile = this.file.data;
    this.contentUrl = this.fileService.buildThumbnailUrl(this.file.data, 250, 250);
  }
}
