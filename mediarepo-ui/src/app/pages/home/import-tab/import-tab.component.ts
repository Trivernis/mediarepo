import { Component, OnInit } from '@angular/core';
import {File} from "../../../models/File";

@Component({
  selector: 'app-import-tab',
  templateUrl: './import-tab.component.html',
  styleUrls: ['./import-tab.component.scss']
})
export class ImportTabComponent {

  public files: File[] = [];

  constructor() { }

  public addFileFromImport(file: File) {
    this.files.push(file);
    this.files = [...this.files];
  }

}
