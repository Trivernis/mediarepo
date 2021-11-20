import {
  AfterViewChecked,
  Component,
  ElementRef,
  EventEmitter,
  Input,
  OnInit,
  Output,
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
import {SortDialogComponent} from "./sort-dialog/sort-dialog.component";
import {ErrorBrokerService} from "../../services/error-broker/error-broker.service";
import {
  FilterExpression,
  SingleFilterExpression
} from "../../models/FilterExpression";
import {FilterDialogComponent} from "./filter-dialog/filter-dialog.component";
import {Tag} from "../../models/Tag";


@Component({
  selector: 'app-file-search',
  templateUrl: './file-search.component.html',
  styleUrls: ['./file-search.component.scss']
})
export class FileSearchComponent implements AfterViewChecked, OnInit {
  public sortExpression: SortKey[] = [new SortKey("FileImportedTime",
    "Ascending", undefined)];
  public formControl = new FormControl();
  public filters: FilterExpression[] = [];
  public suggestionTags: Observable<string[]>;

  @Input() availableTags: Tag[] = [];
  @Input() validTags: string[] = [];
  @Output() searchStartEvent = new EventEmitter<void>();
  @Output() searchEndEvent = new EventEmitter<void>();

  @ViewChild("tagInput") tagInput!: ElementRef<HTMLInputElement>;
  @ViewChild("tagInputList") inputList!: ElementRef;

  constructor(private errorBroker: ErrorBrokerService, private tagService: TagService, private fileService: FileService, public dialog: MatDialog) {
    this.suggestionTags = this.formControl.valueChanges.pipe(startWith(null),
      map(
        (tag: string | null) => tag ? this.filterSuggestionTag(
          tag) : this.validTags.slice(0, 20)));
  }

  public async ngOnInit() {
    await this.searchForFiles();
  }

  public ngAfterViewChecked(): void {
    this.inputList.nativeElement.scrollLeft = this.inputList.nativeElement.scrollWidth;
  }

  public async searchForFiles() {
    this.searchStartEvent.emit();
    try {
      await this.fileService.findFiles(this.filters, this.sortExpression);
    } catch (err) {
      this.errorBroker.showError(err);
    }
    this.searchEndEvent.emit();
  }

  public addSearchTag(tag: string) {
    this.filters.push(new SingleFilterExpression(TagQuery.fromString(tag)));
    tag = tag.replace(/^-/g, '');

    if (this.filters.filter(t => t.partiallyEq(tag)).length > 1) {
      const index = this.filters.findIndex(t => t.partiallyEq(tag));
      this.filters.splice(index, 1);
    }
  }

  async removeAllSearchTags() {
    this.filters = [];
    await this.searchForFiles();
  }

  async removeFilterExpression(expr: FilterExpression) {
    const index = this.filters.indexOf(expr);
    if (index >= 0) {
      this.filters.splice(index, 1);
    }
    await this.searchForFiles();
  }

  async addSearchTagByInput(event: KeyboardEvent) {
    if (event.key === "Enter") {
      const tag = (this.formControl.value as string ?? "").trim();
      if (tag.length > 0 && this.validTags.includes(tag.replace(/-/g, ''))) {
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
    const sortEntries = this.sortExpression.map(
      key => JSON.parse(JSON.stringify(key))).map(
      key => new SortKey(key.sortType, key.sortDirection, key.namespaceName))
    const openedDialog = this.dialog.open(SortDialogComponent, {
      minWidth: "40vw",
      data: {
        sortEntries,
      },
      disableClose: true,
    });
    openedDialog.afterClosed().subscribe(async (sortExpression) => {
      if (sortExpression) {
        this.sortExpression = sortExpression;
        await this.searchForFiles();
      }
    });
  }

  private filterSuggestionTag(tag: string) {
    const negated = tag.startsWith("-");
    const normalizedTag = tag.replace(/^-/, "");

    return this.validTags.filter(
        t => t.includes(normalizedTag) && this.filters.findIndex(
          f => f.eq(t)) < 0)
      .map(t => negated ? "-" + t : t)
      .slice(0, 20);
  }

  public openFilterDialog(): void {
    const filterEntries = this.filters.map(f => f.clone());
    const filterDialog = this.dialog.open(FilterDialogComponent, {
      minWidth: "25vw",
      height: "80vh",
      data: {
        filterEntries,
        availableTags: this.availableTags,
      },
      disableClose: true,
    });
    filterDialog.afterClosed().subscribe(async (filterExpression) => {
      if (filterExpression !== undefined || filterExpression?.length > 0) {
        this.filters = filterExpression;
        await this.searchForFiles();
      }
    });
  }
}
