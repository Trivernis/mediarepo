import {
  Component,
  EventEmitter,
  HostListener,
  Input,
  OnInit,
  Output, QueryList, ViewChildren
} from '@angular/core';
import {File} from "../../models/File";
import {FileService} from "../../services/file/file.service";
import {FileGridEntryComponent} from "./file-grid-entry/file-grid-entry.component";

@Component({
  selector: 'app-file-grid',
  templateUrl: './file-grid.component.html',
  styleUrls: ['./file-grid.component.scss']
})
export class FileGridComponent {

  @Input() fileRows: File[][] = [];
  @Output() fileDblClickEvent = new EventEmitter<File>();
  @Output() filesSelectEvent = new EventEmitter<File[]>();

  @ViewChildren(FileGridEntryComponent) childQuery!: QueryList<FileGridEntryComponent>;

  selectedEntries: FileGridEntryComponent[] = [];

  private shiftClicked = false;
  private ctrlClicked = false;

  constructor() { }

  /**
   * File selector logic
   * @param {FileGridEntryComponent} entry
   */
  setSelectedFile(entry: FileGridEntryComponent) {
    if (!(this.shiftClicked || this.ctrlClicked) && this.selectedEntries.length > 0) {
      this.selectedEntries.forEach(entry => entry.selected = false);
      this.selectedEntries = [];
    }
    // shift selector (forwards and backwards)
    if (this.shiftClicked && this.selectedEntries.length > 0) {
      const lastEntry = this.selectedEntries[this.selectedEntries.length - 1];
      let found = false;

      // TODO: change to use wrapped entry files instead because of reused components
      for (const child of this.childQuery) {

        if (found) {
          child.selected = true;
          this.selectedEntries.push(child);
          if (child === entry || child == lastEntry) {
            break;
          }
        } else if (child === lastEntry || child === entry) {
          found = true;
          if (child === entry) {
            child.selected = true;
            this.selectedEntries.push(child);
          }
        }

      }
    } else {
      entry.selected = true;
      this.selectedEntries.push(entry);
    }
    this.filesSelectEvent.emit(this.selectedEntries.map(entry => entry.file));
  }

  @HostListener("window:keydown", ["$event"])
  private handleKeydownEvent(event: KeyboardEvent) {
    switch (event.key) {
      case "Shift": this.shiftClicked = true; break;
      case "Control": this.ctrlClicked = true; break;
    }
  }

  @HostListener("window:keyup", ["$event"])
  private handleKeyupEvent(event: KeyboardEvent) {
    switch (event.key) {
      case "Shift": this.shiftClicked = false; break;
      case "Control": this.ctrlClicked = false; break;
    }
  }
}
