import { Component, OnInit } from '@angular/core';
import {File} from "../../../models/File";

@Component({
  selector: 'app-import-tab',
  templateUrl: './import-tab.component.html',
  styleUrls: ['./import-tab.component.scss']
})
export class ImportTabComponent implements OnInit {

  public files: File[] = [];

  constructor() { }

  ngOnInit(): void {
  }

}
