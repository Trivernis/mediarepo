import {
  Component, ElementRef,
  EventEmitter, HostListener,
  Input,
  OnChanges,
  OnInit,
  Output, SimpleChanges, ViewChild
} from '@angular/core';
import {File} from "../../models/File";
import {FileService} from "../../services/file/file.service";
import {SafeResourceUrl} from "@angular/platform-browser";
import {Selectable} from "../../models/Selectable";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";

@Component({
  selector: 'app-file-gallery',
  templateUrl: './file-gallery.component.html',
  styleUrls: ['./file-gallery.component.scss']
})
export class FileGalleryComponent implements OnChanges, OnInit {

  @Input() files: File[] = [];
  @Input() preselectedFile: File | undefined;
  @Output() fileSelectEvent = new EventEmitter<File | undefined>();
  @Output() fileDblClickEvent = new EventEmitter<File>();
  @Output() closeEvent = new EventEmitter<void>();
  entries: Selectable<File>[] = [];

  @ViewChild("virtualScroll") virtualScroll!: CdkVirtualScrollViewport;

  selectedFile: Selectable<File> | undefined;
  fileContentUrl: SafeResourceUrl | undefined;

  constructor(private fileService: FileService) {
  }

  /**
   * Called when a new entry is selected
   * @param {Selectable<File>} entry
   * @returns {Promise<void>}
   */
  async onEntrySelect(entry: Selectable<File>) {
    if (entry) {
      this.selectedFile?.unselect();
      entry.select();
      this.selectedFile = entry;
      await this.loadSelectedFile();
      this.virtualScroll.scrollToIndex(this.entries.indexOf(entry), "smooth");
      this.fileSelectEvent.emit(this.selectedFile.data);
    }
  }

  /**
   * Loads the content url of the selected file
   * @returns {Promise<void>}
   */
  async loadSelectedFile() {
    if (this.selectedFile) {
      this.fileContentUrl = await this.fileService.readFile(this.selectedFile.data);
    }
  }

  async ngOnInit(): Promise<void> {
    if (!this.selectedFile || this.files.indexOf(this.selectedFile.data) < 0) {
      await this.onEntrySelect(this.getPreselectedEntry() ?? this.entries[0])
    }
  }

  public async ngOnChanges(changes: SimpleChanges): Promise<void> {
    this.entries = this.files.map(f => new Selectable(f, f.hash == this.selectedFile?.data.hash));
    const selectedIndex = this.files.findIndex(f => f.hash === this.selectedFile?.data.hash);

    if (!this.selectedFile || selectedIndex < 0) {
      await this.onEntrySelect(this.getPreselectedEntry() ?? this.entries[0])
    } else {
      await this.onEntrySelect(this.entries[selectedIndex])
    }
  }

  /**
   * Selects the previous item in the gallery
   * @returns {Promise<void>}
   */
  public async nextItem() {
    if (this.selectedFile) {
      let index = this.entries.indexOf(this.selectedFile) + 1;
      if (index == this.entries.length) {
        index--;  // restrict to elements
      }
      await this.onEntrySelect(this.entries[index]);
    } else {
      await this.onEntrySelect(this.entries[0])
    }
  }

  /**
   * Selects the next item in the gallery
   * @returns {Promise<void>}
   */
  public async previousItem() {
    if (this.selectedFile) {
      let index = this.entries.indexOf(this.selectedFile) - 1;
      if (index < 0) {
        index++; // restrict to elements
      }
      await this.onEntrySelect(this.entries[index]);
    } else {
      await this.onEntrySelect(this.entries[0])
    }
  }

  @HostListener("window:keydown", ["$event"])
  private async handleKeydownEvent(event: KeyboardEvent) {
    switch (event.key) {
      case "ArrowRight":
        await this.nextItem();
        break;
      case "ArrowLeft":
        await this.previousItem();
        break;
    }
  }

  private getPreselectedEntry(): Selectable<File> | undefined {
    if (this.preselectedFile) {
      const entry = this.entries.find(e => e.data.hash == this.preselectedFile?.hash);
      if (entry) {
        return entry;
      }
    }
    return undefined;
  }
}
