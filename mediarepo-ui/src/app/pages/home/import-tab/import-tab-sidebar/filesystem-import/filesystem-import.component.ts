import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-filesystem-import',
  templateUrl: './filesystem-import.component.html',
  styleUrls: ['./filesystem-import.component.scss']
})
export class FilesystemImportComponent {

  public fileCount: number = 0;
  public paths: string[] = [];
  public importTagsFromTxt = true;
  public deleteAfterImport = false;

  constructor() { }

  public async setSelectedPaths(paths: string[]) {
    this.paths = paths;
    this.fileCount = paths.length;
  }
}
