import { Component, OnInit } from '@angular/core';
import {FileService} from "../../services/file/file.service";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  constructor(private fileService: FileService) { }

  async ngOnInit() {
    await this.fileService.getFiles();
  }

}
