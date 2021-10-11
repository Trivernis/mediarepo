import { Component, OnInit } from '@angular/core';
import {File} from "../../models/File";
import {FileService} from "../../services/file/file.service";

@Component({
  selector: 'app-file-grid',
  templateUrl: './file-grid.component.html',
  styleUrls: ['./file-grid.component.scss']
})
export class FileGridComponent implements OnInit {

  files: File[] = [];

  constructor(private fileService: FileService) { }

  ngOnInit(): void {
    this.fileService.displayedFiles.subscribe((files) => this.files = files);
  }

}
