import {Component, ViewChild} from '@angular/core';
import {File} from "../../../models/File";
import {FileService} from "../../../services/file/file.service";
import {FileGridComponent} from "../../../components/file-grid/file-grid.component";
import {DialogFilter} from "@tauri-apps/api/dialog";

@Component({
  selector: 'app-import-tab',
  templateUrl: './import-tab.component.html',
  styleUrls: ['./import-tab.component.scss']
})
export class ImportTabComponent {

  public files: File[] = [];
  @ViewChild("fileGrid") fileGrid!: FileGridComponent;

  constructor(private fileService: FileService) {
  }

  /**
   * Adds an imported file to the list of imported files
   * @param {File} file
   * @returns {Promise<void>}
   */
  public async addFileFromImport(file: File) {
    this.files.push(file);
    if (this.files.length % 50 === 0) {  // refresh every 50 pictures
      this.refreshFileView();
    }
  }

  /**
   * Refreshes the file view
   * @returns {Promise<void>}
   */
  public refreshFileView() {
    this.files = [...this.files];
  }
}
