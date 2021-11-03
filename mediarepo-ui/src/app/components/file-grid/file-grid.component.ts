import {
  Component, ElementRef,
  EventEmitter,
  HostListener,
  Input, OnChanges,
  OnInit,
  Output, QueryList, SimpleChanges, ViewChild, ViewChildren
} from '@angular/core';
import {File} from "../../models/File";
import {FileService} from "../../services/file/file.service";
import {FileGridEntryComponent} from "./file-grid-entry/file-grid-entry.component";
import {GridEntry} from "./file-grid-entry/GridEntry";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";

@Component({
  selector: 'app-file-grid',
  templateUrl: './file-grid.component.html',
  styleUrls: ['./file-grid.component.scss']
})
export class FileGridComponent implements OnChanges, OnInit {

  @Input() files: File[] = [];
  @Input() columns: number = 6;
  @Input() preselectedFile: File | undefined;
  @Output() fileDblClickEvent = new EventEmitter<File>();
  @Output() fileMultiselectEvent = new EventEmitter<File[]>();
  @Output() fileSelectEvent = new EventEmitter<File | undefined>();

  @ViewChild("virtualScrollGrid") virtualScroll!: CdkVirtualScrollViewport;
  @ViewChild("galleryWrapper") galleryWrapper!: ElementRef<HTMLDivElement>;

  selectedEntries: GridEntry[] = [];

  private shiftClicked = false;
  private ctrlClicked = false;
  private gridEntries: GridEntry[] = []
  partitionedGridEntries: GridEntry[][] = [];

  constructor() {
  }

  public ngOnInit(): void {
    this.gridEntries = this.files.map(file => {return {file, selected: false}});
    this.setPartitionedGridEntries();
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes["files"]) {
      this.gridEntries = this.files.map(file => {return {file, selected: false}});
      this.setPartitionedGridEntries();
    }
  }

  /**
   * File selector logic
   * @param {FileGridEntryComponent} clickedEntry
   */
  setSelectedFile(clickedEntry: GridEntry) {
    const previousSelectionSize = this.selectedEntries.length;

    if (!(this.shiftClicked || this.ctrlClicked) && this.selectedEntries.length > 0) {
      this.selectedEntries.forEach(entry => {if (entry !== clickedEntry) entry.selected = false});
      this.selectedEntries = [];
    }
    if (this.shiftClicked && this.selectedEntries.length > 0) {
      this.handleShiftSelect(clickedEntry);
    } else {
      clickedEntry.selected = !clickedEntry.selected;
      if (!clickedEntry.selected) {
        const index = this.selectedEntries.indexOf(clickedEntry);
        if (index > -1) {
          this.selectedEntries.splice(index, 1);
        }
      } else {
        this.selectedEntries.push(clickedEntry);
      }
    }
    if (this.selectedEntries.length == 1) {
      this.fileSelectEvent.emit(this.selectedEntries.map(entry => entry.file)[0]);
    } else if (this.selectedEntries.length == 0 && previousSelectionSize == 1){
      this.fileSelectEvent.emit(undefined);
    } else {
      this.fileMultiselectEvent.emit(this.selectedEntries.map(entry => entry.file));
    }
  }

  private setPartitionedGridEntries() {
    this.partitionedGridEntries = [];
    this.selectedEntries = [];
    let scrollToIndex = -1;
    let selectedEntry: GridEntry | undefined = undefined;

    for (let i = 0; i < (Math.ceil(this.gridEntries.length / this.columns)); i++) {
      const entries = this.gridEntries.slice(i * this.columns, Math.min(this.gridEntries.length, (i + 1) * this.columns));
      this.partitionedGridEntries.push(entries);
      const preselectedEntry = entries.find(e => e.file.hash == this.preselectedFile?.hash);

      if (preselectedEntry) {
        scrollToIndex = i;
        selectedEntry = preselectedEntry;
      }
    }
    if (scrollToIndex >= 0 && this.preselectedFile && this.selectedEntries.length == 0) {
      setTimeout(() => {  // add timeout to avoid being stuck in the update loop
        if (this.virtualScroll) {
          this.virtualScroll?.scrollToIndex(scrollToIndex);
          if (selectedEntry) {
            selectedEntry.selected = true;
            this.selectedEntries = [selectedEntry];
          }
        }
      }, 0);
    }
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
