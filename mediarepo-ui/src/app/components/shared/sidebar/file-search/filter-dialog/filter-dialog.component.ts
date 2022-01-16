import {Component, Inject, OnChanges, SimpleChanges} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {SortDialogComponent} from "../sort-dialog/sort-dialog.component";
import {Tag} from "../../../../../../api/models/Tag";
import {SearchFilters} from "../../../../../../api/models/SearchFilters";
import {FilterExpression, FilterQuery} from "../../../../../../api/api-types/files";
import {enumerate, removeByValue} from "../../../../../utils/list-utils";

type IndexableSelection<T> = {
    [key: number]: T
};

@Component({
    selector: "app-filter-dialog",
    templateUrl: "./filter-dialog.component.html",
    styleUrls: ["./filter-dialog.component.scss"]
})
export class FilterDialogComponent implements OnChanges {
    public availableTags: Tag[] = [];
    public filters = new SearchFilters([]);
    public renderedFilterEntries: [number, FilterExpression][] = [];
    private selectedIndices: IndexableSelection<number[]> = {};

    constructor(public dialogRef: MatDialogRef<SortDialogComponent>, @Inject(
        MAT_DIALOG_DATA) data: any) {
        this.availableTags = data.availableTags ?? [];
        this.filters = data.filters;
        this.buildRenderedEntries();
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["filters"]) {
            this.buildRenderedEntries();
        }
    }

    public cancelFilter(): void {
        this.dialogRef.close();
    }

    public confirmFilter(): void {
        this.dialogRef.close(this.filters);
    }

    public entrySelect(index: number, subindex: number = -1): void {
        this.selectedIndices[index] = this.selectedIndices[index] ?? [];
        this.selectedIndices[index].push(subindex);
    }

    public entryUnselect(index: number, subindex: number = -1): void {
        this.selectedIndices[index] = this.selectedIndices[index] ?? [];
        removeByValue(this.selectedIndices[index], subindex);
    }

    public addFilter(expression: FilterExpression): void {
        this.filters.addFilterExpression(expression);
        this.buildRenderedEntries();
    }

    public removeSelectedFilters(): void {
        const orderedIndices = Object.keys(this.selectedIndices).map(k => Number(k)).sort().reverse();

        for (const indexStr of orderedIndices) {
            const index = indexStr;
            const subIndices: number[] = this.selectedIndices[index];

            if (subIndices.length === 1 && subIndices[0] === -1) {
                this.filters.removeFilterAtIndex(index);
            } else if (subIndices.length > 0) {
                for (const subIndex of subIndices.sort().reverse()) {   // need to remove from the top down to avoid index shifting
                    this.filters.removeSubfilterAtIndex(index, subIndex);
                }
            }
        }
        this.selectedIndices = {};
        this.buildRenderedEntries();
    }

    public createAndFromSelection(deleteOriginal: boolean): void {
        const expressions: FilterExpression[] = [];

        for (const indexStr in this.selectedIndices) {
            const index = Number(indexStr);
            const subindices = this.selectedIndices[index];

            if (subindices.length === 1 && subindices[0] === -1) {
                expressions.push(this.filters.getFilters()[index]);
            } else {
                for (const subIndex of subindices) {
                    const query = this.filters.getSubfilterAtIndex(index, subIndex);
                    if (query) {
                        expressions.push({ Query: query });
                    }
                }
            }
        }
        if (deleteOriginal) {
            this.removeSelectedFilters();
        } else {
            this.selectedIndices = {};
        }
        expressions.forEach(e => this.filters.addFilterExpression(e));
        this.buildRenderedEntries();
    }

    public createOrFromSelection(deleteOriginal: boolean): void {
        const queries: FilterQuery[] = [];

        for (const indexStr in this.selectedIndices) {
            const index = Number(indexStr);
            const subindices = this.selectedIndices[index];

            if (subindices.length === 1 && subindices[0] === -1) {
                const filterEntry = this.filters.getFilters()[index];
                if ("Query" in filterEntry) {
                    queries.push(filterEntry.Query);
                }
            } else {
                for (const subIndex of subindices) {
                    const query = this.filters.getSubfilterAtIndex(index, subIndex);
                    if (query) {
                        queries.push(query);
                    }
                }
            }
        }
        if (deleteOriginal) {
            this.removeSelectedFilters();
        } else {
            this.selectedIndices = {};
        }
        if (queries.length > 1) {
            this.filters.addFilterExpression({ OrExpression: queries });
        } else if (queries.length === 1) {
            this.filters.addFilterExpression({ Query: queries[0] });
        }
        this.buildRenderedEntries();
    }

    private buildRenderedEntries() {
        this.renderedFilterEntries = enumerate(this.filters.getFilters());
    }
}
