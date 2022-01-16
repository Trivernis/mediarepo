import {FilterExpression, FilterExpressionQuery, FilterQuery} from "../api-types/files";
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

    public getSubfilterAtIndex(index: number, subindex: number): FilterQuery | undefined {
        if (index < this.filters.length) {
            const filterEntry = this.filters[index]!;
            if ("OrExpression" in filterEntry) {
                return filterEntry.OrExpression[subindex];
            }
        }
        return undefined;
    }

    public hasFilter(expression: FilterExpression): boolean {
        return !!this.filters.find(f => deepEqual(f, expression));
    }

    public hasSubfilter(query: FilterQuery): boolean {
        return !!this.filters.find(f => {
            if ("OrExpression" in f) {
                return !!f.OrExpression.find(q => deepEqual(q, query));
            } else {
                return deepEqual(f.Query, query);
            }
        });
    }

    public addFilterExpression(filter: FilterExpression) {
        this.filters.push(filter);
        this.processChangesToOrExpressions();
    }

    public addFilter(filter: FilterExpression, index: number) {
        this.filters = [...this.filters.slice(
            0,
            index
        ), filter, ...this.filters.slice(index)];
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
            delete entry["Query"];
            entry["OrExpression"] = [otherQuery, filter];
        }
    }

    public removeSubfilter(queryToRemove: FilterQuery) {
        let index = this.filters.findIndex(f => {
            if ("Query" in f) {
                return false;
            } else {
                f["OrExpression"] = f["OrExpression"]!.filter(q => !deepEqual(q, queryToRemove));
                return (!f["OrExpression"] || f["OrExpression"]!.length === 0);
            }
        });
        if (index >= 0) {
            this.filters.splice(index, 1);
        }
        this.processChangesToOrExpressions();
    }

    public removeSubfilterAtIndex(index: number, subindex: number) {
        const filterEntry = this.filters[index];

        if (filterEntry && "OrExpression" in filterEntry) {
            filterEntry["OrExpression"]!.splice(subindex, 1);

            if (filterEntry["OrExpression"]!.length === 0) {
                this.removeFilterAtIndex(index);
            }
        }
        this.processChangesToOrExpressions();
    }

    private processChangesToOrExpressions() {
        const filters_to_remove: FilterExpression[] = [];

        for (const filter of this.filters) {
            if ("OrExpression" in filter && !("Query" in filter)) {
                if (filter.OrExpression && filter.OrExpression.length === 1) {
                    const query = filter.OrExpression[0];
                    let newFilter = filter as unknown as FilterExpressionQuery & { OrExpression: undefined };
                    delete newFilter["OrExpression"];
                    newFilter.Query = query;
                } else if (!filter.OrExpression || filter.OrExpression.length === 0) {
                    filters_to_remove.push(filter);
                }
            }
        }
        filters_to_remove.forEach(f => this.removeFilter(f));
    }
}
