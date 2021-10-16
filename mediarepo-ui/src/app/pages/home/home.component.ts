import { Component, OnInit } from '@angular/core';
import {FileService} from "../../services/file/file.service";
import {File} from "../../models/File";
import {PageEvent} from "@angular/material/paginator";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  fileRows: File[][] = [];
  page: number = 0;
  pageSize: number = 25;

  constructor(private fileService: FileService) { }

  async ngOnInit() {
    this.fileService.displayedFiles.subscribe((files) => this.setFileRows(files));
    await this.fileService.getFiles();
  }

  setFileRows(files: File[]) {
    this.fileRows = [];
    const filesPerRow = 6;
    for (let i = 0; i < (Math.ceil(files.length /filesPerRow )); i++) {
      this.fileRows.push(files.slice(i * filesPerRow, Math.min(files.length, (i + 1) * filesPerRow)))
    }
    console.log(this.fileRows);
  }
}
