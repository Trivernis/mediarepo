import {
  Component,
  EventEmitter,
  OnInit,
  Output,
  ViewChild
} from '@angular/core';
import {MatTabGroup} from "@angular/material/tabs";
import {File} from "../../../../models/File";

@Component({
  selector: 'app-import-tab-sidebar',
  templateUrl: './import-tab-sidebar.component.html',
  styleUrls: ['./import-tab-sidebar.component.scss']
})
export class ImportTabSidebarComponent {

  @Output() fileImported = new EventEmitter<File>();
  @Output() importFinished = new EventEmitter<void>();

  constructor() { }
}
