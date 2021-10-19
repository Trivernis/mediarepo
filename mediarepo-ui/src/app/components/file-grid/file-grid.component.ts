import {
  Component,
  EventEmitter,
  HostListener,
  Input, OnChanges,
  OnInit,
  Output, QueryList, SimpleChanges, ViewChildren
} from '@angular/core';
import {File} from "../../models/File";
import {FileService} from "../../services/file/file.service";
import {FileGridEntryComponent} from "./file-grid-entry/file-grid-entry.component";
import {GridEntry} from "./file-grid-entry/GridEntry";

@Component({
  selector: 'app-file-grid',
  templateUrl: './file-grid.component.html',
  styleUrls: ['./file-grid.component.scss']
})
export class FileGridComponent implements OnChanges {

  @Input() files: File[] = [];
  @Input() columns: number = 6;
  @Output() fileDblClickEvent = new EventEmitter<File>();
  @Output() filesSelectEvent = new EventEmitter<File[]>();

  selectedEntries: GridEntry[] = [];

  private shiftClicked = false;
  private ctrlClicked = false;
  private gridEntries: GridEntry[] = []
  partitionedGridEntries: GridEntry[][] = [];

  constructor() {
  }

  ngOnChanges(changes: SimpleChanges): void {
    this.gridEntries = this.files.map(file => {return {file, selected: false}});
    this.setPartitionedGridEntries();
  }

  private setPartitionedGridEntries() {
    this.partitionedGridEntries = [];

    for (let i = 0; i < (Math.ceil(this.gridEntries.length / this.columns)); i++) {
      this.partitionedGridEntries.push(this.gridEntries.slice(i * this.columns, Math.min(this.gridEntries.length, (i + 1) * this.columns)))
    }
  }

  /**
   * File selector logic
   * @param {FileGridEntryComponent} clickedEntry
   */
  setSelectedFile(clickedEntry: GridEntry) {
    if (!(this.shiftClicked || this.ctrlClicked) && this.selectedEntries.length > 0) {
      this.selectedEntries.forEach(entry => {if (entry !== clickedEntry) entry.selected = false});
      this.selectedEntries = [];
    }
    if (this.shiftClicked && this.selectedEntries.length > 0) {
      this.handleShiftSelect(clickedEntry);
    } else {
      clickedEntry.selected = !clickedEntry.selected;
      this.selectedEntries.push(clickedEntry);
    }
    this.filesSelectEvent.emit(this.selectedEntries.map(entry => entry.file));
  }

  private handleShiftSelect(clickedEntry: GridEntry): void {
    const lastEntry = this.selectedEntries[this.selectedEntries.length - 1];
    let found = false;
    if (clickedEntry == lastEntry) {
      return;
    }

    for (const gridEntry of this.gridEntries) {
      if (found) {
        gridEntry.selected = true;
        this.selectedEntries.push(gridEntry);
        if (gridEntry === clickedEntry || gridEntry == lastEntry) {
          return;
        }
      } else if (gridEntry === lastEntry || gridEntry === clickedEntry) {
        found = true;
        if (gridEntry === clickedEntry) {
          gridEntry.selected = true;
          this.selectedEntries.push(gridEntry);
        }
      }

    }
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
