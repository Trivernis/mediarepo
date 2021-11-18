import {
  Component,
  ElementRef,
  HostListener,
  Inject,
  ViewChild
} from '@angular/core';
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {SortDialogComponent} from "../sort-dialog/sort-dialog.component";
import {
  FilterExpression, OrFilterExpression,
  SingleFilterExpression
} from "../../../models/FilterExpression";
import {Observable} from "rxjs";
import {FormControl} from "@angular/forms";
import {last, map, startWith} from "rxjs/operators";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {TagQuery} from "../../../models/TagQuery";

@Component({
  selector: 'app-filter-dialog',
  templateUrl: './filter-dialog.component.html',
  styleUrls: ['./filter-dialog.component.scss']
})
export class FilterDialogComponent {

  public filters: FilterExpression[];
  public suggestionTags: Observable<string[]>;
  public validTags: string[] = [];
  public formControl = new FormControl();
  public mode: "AND" | "OR" = "AND";
  @ViewChild("tagInput") tagInput!: ElementRef<HTMLInputElement>;

  constructor(public dialogRef: MatDialogRef<SortDialogComponent>, @Inject(
    MAT_DIALOG_DATA) data: any) {
    this.filters = data.filterEntries;
    this.validTags = data.validTags;

    this.suggestionTags = this.formControl.valueChanges.pipe(startWith(null),
      map(
        (tag: string | null) => tag ? this.filterSuggestionTag(
          tag) : this.validTags.slice(0, 20)));
  }

  public cancelFilter(): void {
    this.dialogRef.close();
  }

  public confirmFilter(): void {
    this.dialogRef.close(this.filters);
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

  public addFilterByAutocomplete(event: MatAutocompleteSelectedEvent): void {
    this.addFilter(event.option.value);
    this.formControl.setValue(null);
    this.tagInput.nativeElement.value = '';
  }

  public addFilterByInput(): void {
    this.addFilter(this.formControl.value);
    this.formControl.setValue(null);
    this.tagInput.nativeElement.value = '';
  }

  public addFilter(tag: string) {
    const query = TagQuery.fromString(tag);

    if (this.mode === "AND") {
      this.filters.push(new SingleFilterExpression(query));
      tag = tag.replace(/^-/g, '');

      if (this.filters.filter(t => t.partiallyEq(tag)).length > 1) {
        const index = this.filters.findIndex(t => t.partiallyEq(tag));
        this.filters.splice(index, 1);
      }
    } else {
      let queryList = this.filters.pop()?.queryList() ?? [];

      queryList.push(query);
      this.filters.push(new OrFilterExpression(queryList));
    }
  }

  @HostListener("window:keydown", ["$event"])
  private async handleKeydownEvent(event: KeyboardEvent) {
    if (event.key === "Shift") {
      this.mode = "OR";
    }
  }

  @HostListener("window:keyup", ["$event"])
  private async handleKeyupEvent(event: KeyboardEvent) {
    if (event.key === "Shift") {
      this.mode = "AND";
    }
  }
}
