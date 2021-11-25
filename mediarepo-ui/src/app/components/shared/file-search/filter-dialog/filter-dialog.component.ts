import {Component, HostListener, Inject, ViewChildren} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {SortDialogComponent} from "../sort-dialog/sort-dialog.component";
import {
    FilterExpression,
    OrFilterExpression,
    SingleFilterExpression
} from "../../../../models/FilterExpression";
import {TagQuery} from "../../../../models/TagQuery";
import {Tag} from "../../../../models/Tag";
import {TagFilterListItemComponent} from "./tag-filter-list-item/tag-filter-list-item.component";
import {Selectable} from "../../../../models/Selectable";

@Component({
    selector: "app-filter-dialog",
    templateUrl: "./filter-dialog.component.html",
    styleUrls: ["./filter-dialog.component.scss"]
})
export class FilterDialogComponent {

    public filters: Selectable<FilterExpression>[];
    public availableTags: Tag[] = [];
    public mode: "AND" | "OR" = "AND";

    @ViewChildren(
        TagFilterListItemComponent) filterListItems!: TagFilterListItemComponent[];

    private selectedQueries: TagQuery[] = [];

    constructor(public dialogRef: MatDialogRef<SortDialogComponent>, @Inject(
        MAT_DIALOG_DATA) data: any) {
        this.filters = data.filterEntries.map(
            (f: FilterExpression) => new Selectable<FilterExpression>(f,
                false)) ?? [];
        this.availableTags = data.availableTags ?? [];
    }

    private static checkFiltersEqual(l: FilterExpression, r: FilterExpression): boolean {
        const lTags = l.queryList().map(q => q.getNormalizedTag()).sort();
        const rTags = r.queryList().map(q => q.getNormalizedTag()).sort();
        let match = false;

        if (lTags.length == rTags.length) {
            match = true;

            for (const tag of lTags) {
                match = rTags.includes(tag);
                if (!match) {
                    break;
                }
            }
        }
        return match;
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
        this.unselectAll();
    }

    public addFilter(tag: string) {
        const query = TagQuery.fromString(tag);

        if (this.mode === "AND" || this.filters.length === 0) {
            this.filters.push(
                new Selectable<FilterExpression>(
                    new SingleFilterExpression(query),
                    false));
            tag = tag.replace(/^-/g, "");

            if (this.filters.filter(t => t.data.partiallyEq(tag)).length > 1) {
                const index = this.filters.findIndex(
                    t => t.data.partiallyEq(tag));
                this.filters.splice(index, 1);
            }
        } else {
            let queryList = this.filters.pop()?.data.queryList() ?? [];

            queryList.push(query);
            const filterExpression = new OrFilterExpression(queryList);
            filterExpression.removeDuplicates();
            this.filters.push(
                new Selectable<FilterExpression>(filterExpression,
                    false));
        }
        this.unselectAll();
    }

    public addToSelection(query: TagQuery): void {
        this.selectedQueries.push(query);
    }

    public removeFromSelection(query: TagQuery): void {
        const index = this.selectedQueries.indexOf(query);
        if (index > 0) {
            this.selectedQueries.splice(index, 1);
        }
    }

    public unselectAll() {
        this.filters.forEach(filter => filter.selected = false);
        this.selectedQueries = [];
        this.filterListItems.forEach(i => i.selectedIndices = []);
    }

    public convertSelectionToAndExpression(): void {
        for (const query of this.selectedQueries) {
            this.filters.push(
                new Selectable<FilterExpression>(
                    new SingleFilterExpression(query),
                    false));
        }
        this.removeFilterDuplicates();
        this.unselectAll();
    }

    public convertSelectionToOrExpression(): void {
        const queries = this.selectedQueries;
        const expression = new OrFilterExpression(queries);
        this.filters.push(new Selectable<FilterExpression>(expression, false));
        this.removeFilterDuplicates();
        this.unselectAll();
    }

    public invertSelection(): void {
        this.selectedQueries.forEach(query => query.negate = !query.negate);
    }

    private removeFilterDuplicates() {
        const filters = this.filters;
        let newFilters: Selectable<FilterExpression>[] = [];

        for (const filterItem of filters) {
            if (filterItem.data.filter_type == "OrExpression") {
                (filterItem.data as OrFilterExpression).removeDuplicates();
            }
            if (newFilters.findIndex(
                f => FilterDialogComponent.checkFiltersEqual(f.data,
                    filterItem.data)) < 0) {
                if (filterItem.data.filter_type == "OrExpression" && filterItem.data.queryList().length === 1) {
                    filterItem.data = new SingleFilterExpression(
                        filterItem.data.queryList()[0]);
                }
                newFilters.push(filterItem);
            }
        }
        this.filters = newFilters;
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
