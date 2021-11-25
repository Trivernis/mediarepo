import {
  Component,
  ElementRef,
  EventEmitter,
  Input,
  OnChanges,
  OnInit,
  Output,
  SimpleChanges,
  ViewChild
} from '@angular/core';
import {File} from "../../../../../models/File";
import {FileService} from "../../../../../services/file/file.service";
import {ErrorBrokerService} from "../../../../../services/error-broker/error-broker.service";
import {SafeResourceUrl} from "@angular/platform-browser";
import {GridEntry} from "./GridEntry";

@Component({
  selector: 'app-file-grid-entry',
  templateUrl: './file-grid-entry.component.html',
  styleUrls: ['./file-grid-entry.component.scss']
})
export class FileGridEntryComponent implements OnInit, OnChanges {

  @ViewChild("card") card!: ElementRef;
  @Input() public gridEntry!: GridEntry;
  @Output() clickEvent = new EventEmitter<FileGridEntryComponent>();
  @Output() dblClickEvent = new EventEmitter<FileGridEntryComponent>();

  contentUrl: SafeResourceUrl | undefined;
  private cachedFile: File | undefined;
  private urlSetTimeout: number | undefined;

  constructor(private fileService: FileService, private errorBroker: ErrorBrokerService) {
  }

  async ngOnInit() {
    this.cachedFile = this.gridEntry.file;
    this.setImageDelayed();
  }

  async ngOnChanges(changes: SimpleChanges) {
    if (changes["file"] && (!this.cachedFile || this.gridEntry.file.hash !== this.cachedFile.hash)) {
      this.cachedFile = this.gridEntry.file;
      this.setImageDelayed();
    }
  }

  private setImageDelayed() {
    this.contentUrl = undefined;
    clearTimeout(this.urlSetTimeout);
    this.urlSetTimeout = setTimeout(
      () => this.contentUrl = this.fileService.buildThumbnailUrl(
        this.gridEntry.file,
        250, 250), 200);
  }
}
