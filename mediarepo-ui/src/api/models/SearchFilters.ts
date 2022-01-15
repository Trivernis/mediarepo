import {FilterExpression, FilterQuery} from "../api-types/files";
import * as deepEqual from "fast-deep-equal";

export class SearchFilters {
    constructor(private filters: FilterExpression[]) {
    }

    public get length() {
        return this.filters.length;
    }

    public getFilters(): FilterExpression[] {
        return this.filters;
    }

    public hasFilter(expression: FilterExpression): boolean {
        return !!this.filters.find(f => deepEqual(f, expression));
    }

    public addFilterExpression(filter: FilterExpression) {
        this.filters.push(filter);
    }

    public addFilter(filter: FilterQuery, index: number) {
        this.filters = [...this.filters.slice(
            0,
            index
        ), { Query: filter }, ...this.filters.slice(index)];
    }

    public appendFilter(filter: FilterQuery) {
        this.filters.push({ Query: filter });
    }

    public removeFilter(filterToRemove: FilterExpression) {
        this.filters = this.filters.filter(f => !deepEqual(f, filterToRemove));
    }

    public removeFilterAtIndex(index: number) {
        this.filters.splice(index, 1);
    }

    public appendSubfilter(filter: FilterQuery, index: number) {
        const expressionEntry = this.filters[index];

        if (expressionEntry && "OrExpression" in expressionEntry) {
            expressionEntry["OrExpression"]!.push(filter);
        } else {
            const otherQuery = expressionEntry["Query"]!;
            let entry = expressionEntry as unknown as { OrExpression: FilterQuery[], Query: undefined };
            entry["Query"] = undefined;
            entry["OrExpression"] = [otherQuery, filter];
        }
    }

    public removeSubfilter(queryToRemove: FilterQuery) {
        let index = this.filters.findIndex(f => {
            if ("Query" in f) {
                return false;
            } else {
                f["OrExpression"] = f["OrExpression"]!.filter(q => !deepEqual(q, queryToRemove));
                return (f["OrExpression"]!.length === 0);
            }
        });
        this.filters.splice(index);
    }

    public removeSubfilterAtIndex(index: number, subindex: number) {
        const filterEntry = this.filters[index];

        if (filterEntry && "OrExpression" in filterEntry) {
            filterEntry["OrExpression"]!.splice(subindex, 1);

            if (filterEntry["OrExpression"]!.length === 0) {
                this.removeFilterAtIndex(index);
            }
        }
    }
}
