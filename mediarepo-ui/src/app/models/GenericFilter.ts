import {TagQuery} from "./TagQuery";
import {createRustEnum} from "./rust-types";
import {FilterExpression} from "../../api/api-types/files";

export interface GenericFilter {
    filter_type: "OrExpression" | "Query";
    filter: TagQuery[] | TagQuery;

    eq(value: any): boolean;

    partiallyEq(value: any): boolean;

    getDisplayName(): string;

    clone(): GenericFilter;

    queryList(): TagQuery[];

    toBackendType(): FilterExpression;
}

export class OrFilterExpression implements GenericFilter {
    public filter_type: "OrExpression" = "OrExpression";
    public filter: TagQuery[] = [];

    constructor(tags: TagQuery[]) {
        this.filter = tags;
    }

    public eq(value: any): boolean {
        return this == value;
    }

    public partiallyEq(value: any): boolean {
        return this == value;
    }

    public getDisplayName(): string {
        return this.filter.map(t => t.getNormalizedTag()).join(" OR ");
    }

    public clone(): OrFilterExpression {
        let tags = this.filter.map(
            (t: TagQuery) => new TagQuery(t.tag, t.negate));
        return new OrFilterExpression(tags);
    }

    public queryList(): TagQuery[] {
        return this.filter;
    }

    public removeQueryEntry(index: number) {
        this.filter.splice(index, 1);
    }

    public removeDuplicates() {
        const filters = this.filter.reverse();
        let newEntries: TagQuery[] = [];

        for (const entry of filters) {
            if (newEntries.findIndex(f => f.tag === entry.tag) < 0) {
                newEntries.push(entry);
            }
        }
        this.filter = newEntries.reverse();
    }

    public toBackendType(): FilterExpression {
        return createRustEnum(this.filter_type, this.filter) as unknown as FilterExpression;
    }
}

export class SingleFilterExpression implements GenericFilter {
    public filter_type: "Query" = "Query";
    public filter: TagQuery;

    constructor(tag: TagQuery) {
        this.filter = tag;
    }

    public eq(value: any): boolean {
        return (this.filter.tag === value?.name && this.filter.negate === value?.negate) || this.filter.getNormalizedTag() === value;
    }

    public partiallyEq(value: any): boolean {
        return this.filter.tag === value || this.filter.tag === value?.name;
    }

    public getDisplayName(): string {
        return this.filter.getNormalizedTag();
    }

    public clone(): GenericFilter {
        return new SingleFilterExpression(
            new TagQuery(this.filter.tag, this.filter.negate));
    }

    public queryList(): TagQuery[] {
        return [this.filter];
    }

    public toBackendType(): FilterExpression {
        return createRustEnum(this.filter_type, this.filter) as unknown as FilterExpression;
    }
}
