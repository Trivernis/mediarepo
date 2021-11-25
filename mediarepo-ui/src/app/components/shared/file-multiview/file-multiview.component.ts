import {Component, EventEmitter, Input, Output} from '@angular/core';
import {File} from "../../../models/File";

@Component({
  selector: 'app-file-multiview',
  templateUrl: './file-multiview.component.html',
  styleUrls: ['./file-multiview.component.scss']
})
export class FileMultiviewComponent {

  @Input() files!: File[];
  @Input() mode: "grid" | "gallery" = "grid";

  @Output() fileOpenEvent = new EventEmitter<File>();
  @Output() fileSelectEvent = new EventEmitter<File[]>();

  public selectedFiles: File[] = [];
  public preselectedFile: File | undefined;

  constructor() {
  }

  public onFileSelect(files: File[]): void {
    this.selectedFiles = files;
    this.preselectedFile = files[0];
    this.fileSelectEvent.emit(this.selectedFiles);
  }

  public onSinglefileSelect(file: File | undefined): void {
    if (file) {
      this.selectedFiles = [file];
      this.preselectedFile = file;
    } else {
      this.selectedFiles = [];
    }
    this.fileSelectEvent.emit(this.selectedFiles);
  }

  public onFileOpen(file: File): void {
    this.preselectedFile = file;
    this.mode = "gallery";
    this.fileOpenEvent.emit(file);
  }
}
