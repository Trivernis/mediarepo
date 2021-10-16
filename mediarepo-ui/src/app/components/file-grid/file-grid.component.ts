import {Component, Input, OnInit} from '@angular/core';
import {File} from "../../models/File";
import {FileService} from "../../services/file/file.service";

@Component({
  selector: 'app-file-grid',
  templateUrl: './file-grid.component.html',
  styleUrls: ['./file-grid.component.scss']
})
export class FileGridComponent {

  @Input() fileRows: File[][] = [];

  constructor() { }
}
