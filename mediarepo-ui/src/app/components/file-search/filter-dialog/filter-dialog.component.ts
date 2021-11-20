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
import {TagQuery} from "../../../models/TagQuery";
import {Tag} from "../../../models/Tag";
import {TagFilterListItemComponent} from "./tag-filter-list-item/tag-filter-list-item.component";
import {Selectable} from "../../../models/Selectable";

@Component({
  selector: 'app-filter-dialog',
  templateUrl: './filter-dialog.component.html',
  styleUrls: ['./filter-dialog.component.scss']
})
export class FilterDialogComponent {

  public filters: Selectable<FilterExpression>[];
  public availableTags: Tag[] = [];
  public mode: "AND" | "OR" = "AND";

  constructor(public dialogRef: MatDialogRef<SortDialogComponent>, @Inject(
    MAT_DIALOG_DATA) data: any) {
    this.filters = data.filterEntries.map((f: FilterExpression) => new Selectable<FilterExpression>(f, false)) ?? [];
    this.availableTags = data.availableTags ?? [];
  }

  public cancelFilter(): void {
    this.dialogRef.close();
  }

  public confirmFilter(): void {
    this.dialogRef.close(this.filters.map(f => f.data));
  }

  public removeFilter(event: TagFilterListItemComponent): void {
    const filter = event.expression;
    const index = this.filters.findIndex(f => f === filter);
    if (index >= 0) {
      this.filters.splice(index, 1);
    }
  }

  public addFilter(tag: string) {
    const query = TagQuery.fromString(tag);

    if (this.mode === "AND" || this.filters.length === 0) {
      this.filters.push(new Selectable<FilterExpression>(new SingleFilterExpression(query), false));
      tag = tag.replace(/^-/g, '');

      if (this.filters.filter(t => t.data.partiallyEq(tag)).length > 1) {
        const index = this.filters.findIndex(t => t.data.partiallyEq(tag));
        this.filters.splice(index, 1);
      }
    } else {
      let queryList = this.filters.pop()?.data.queryList() ?? [];

      queryList.push(query);
      this.filters.push(new Selectable<FilterExpression>(new OrFilterExpression(queryList), false));
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
