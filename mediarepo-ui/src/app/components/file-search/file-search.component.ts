import {
  AfterViewChecked,
  Component,
  ElementRef, EventEmitter, Output,
  ViewChild
} from '@angular/core';
import {TagService} from "../../services/tag/tag.service";
import {FileService} from "../../services/file/file.service";
import {FormControl} from "@angular/forms";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {map, startWith} from "rxjs/operators";
import {Observable} from "rxjs";
import {TagQuery} from "../../models/TagQuery";
import {SortKey} from "../../models/SortKey";
import {MatDialog} from "@angular/material/dialog";
import {FilterDialogComponent} from "./filter-dialog/filter-dialog.component";
import {ErrorBrokerService} from "../../services/error-broker/error-broker.service";

@Component({
  selector: 'app-file-search',
  templateUrl: './file-search.component.html',
  styleUrls: ['./file-search.component.scss']
})
export class FileSearchComponent implements AfterViewChecked {
  public ngAfterViewChecked(): void {
    this.inputList.nativeElement.scrollLeft = this.inputList.nativeElement.scrollWidth;
  }

  public sortExpression: SortKey[] = [new SortKey("FileImportedTime",
    "Ascending", undefined)];
  public formControl = new FormControl();
  public searchTags: TagQuery[] = [];
  public suggestionTags: Observable<string[]>;

  @Output() searchStartEvent = new EventEmitter<void>();
  @Output() searchEndEvent = new EventEmitter<void>();

  private allTags: string[] = [];

  @ViewChild("tagInput") tagInput!: ElementRef<HTMLInputElement>;
  @ViewChild("tagInputList") inputList!: ElementRef;

  constructor(private errorBroker: ErrorBrokerService, private tagService: TagService, private fileService: FileService, public dialog: MatDialog) {
    this.tagService.tags.subscribe(
      (tag) => this.allTags = tag.map(t => t.getNormalizedOutput()));

    this.suggestionTags = this.formControl.valueChanges.pipe(startWith(null),
      map(
        (tag: string | null) => tag ? this.allTags.filter(
            (t: string) => t.includes(tag.replace(/^-/g, '')))
          .map((t) => tag.startsWith("-") ? "-" + t : t)
          .slice(0, 20) : this.allTags.slice(0, 20)));
  }

  public async searchForFiles() {
    this.searchStartEvent.emit();
    try {
      await this.fileService.findFiles(this.searchTags, this.sortExpression);
    } catch (err) {
      this.errorBroker.showError(err);
    }
    this.searchEndEvent.emit();
  }

  public addSearchTag(tag: string) {
    if (tag.startsWith("-")) {
      tag = tag.replace(/^-/g, '');
      this.searchTags.push(new TagQuery(tag, true));
    } else {
      this.searchTags.push(new TagQuery(tag, false));
    }
    if (this.searchTags.filter(t => t.name === tag).length > 1) {
      const index = this.searchTags.findIndex(t => t.name === tag);
      this.searchTags.splice(index, 1);
    }
  }

  async removeAllSearchTags() {
    this.searchTags = [];
    await this.searchForFiles();
  }

  async removeSearchTag(tag: TagQuery) {
    const index = this.searchTags.indexOf(tag);
    if (index >= 0) {
      this.searchTags.splice(index, 1);
    }
    await this.searchForFiles();
  }

  async addSearchTagByInput(event: KeyboardEvent) {
    if (event.key === "Enter") {
      const tag = (this.formControl.value as string ?? "").trim();
      if (tag.length > 0 && this.allTags.includes(tag.replace(/-/g, ''))) {
        this.addSearchTag(tag);
        this.formControl.setValue(null);
        await this.searchForFiles();
      }
    }
  }

  async addSearchTagByAutocomplete(event: MatAutocompleteSelectedEvent) {
    const tag = event.option.viewValue;
    this.addSearchTag(tag);
    this.formControl.setValue(null);
    this.tagInput.nativeElement.value = '';
    await this.searchForFiles();
  }

  openSortDialog() {
    const sortEntries = this.sortExpression.map(key => JSON.parse(JSON.stringify(key))).map(key => new SortKey(key.sortType, key.sortDirection, key.namespaceName))
    const openedDialog = this.dialog.open(FilterDialogComponent, {
      minWidth: "40vw",
      data: {
        sortEntries,
      },
    });
    openedDialog.afterClosed().subscribe(async (sortExpression) => {
      this.sortExpression = sortExpression;
      await this.searchForFiles();
    });
  }
}
