import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {FormControl} from "@angular/forms";
import {dialog} from "@tauri-apps/api";
import {DialogFilter} from "@tauri-apps/api/dialog";

@Component({
  selector: 'app-native-file-select',
  templateUrl: './native-file-select.component.html',
  styleUrls: ['./native-file-select.component.scss']
})
export class NativeFileSelectComponent implements OnInit{

  @Input() label: string | undefined;
  @Input() mode: "files" | "folders" | "all" = "all";
  @Input() formControlName: string | undefined;
  @Input() formControl: FormControl | undefined;
  @Input() startPath: string | undefined;
  @Input() multiSelect: boolean = true;
  @Input() filters: DialogFilter[] = [];

  @Output() fileSelect = new EventEmitter<string[]>();

  public files: string[] = [];

  constructor() { }

  public ngOnInit(): void {
    if (!this.label) {
      switch (this.mode) {
        case "all":
          this.label = "Select Files or Folders";
          break;
        case "files":
          this.label = "Select Files";
          break;
        case "folders":
          this.label  = "Select a folder";
          break;
      }
    }
  }

  public setFiles(filesExpr: string) {
    this.files = filesExpr.split(",");
    this.fileSelect.emit(this.files);
  }

  /**
   * Opens the native dialog to select files or folders
   * @param {boolean} folders
   * @returns {Promise<void>}
   */
  public async openNativeFileSelectDialog(folders: boolean) {
    const files = await dialog.open({
      multiple: this.multiSelect,
      directory: folders,
      defaultPath: this.startPath,
      filters: this.filters,
    });
    if (files instanceof Array) {
      this.files = files;
    } else {
      this.files = [files];
    }
    this.fileSelect.emit(this.files);
  }

}
