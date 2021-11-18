import {TagQuery} from "./TagQuery";

export interface FilterExpression {
  filter_type: "OrExpression" | "Query";
  filter: TagQuery[] | TagQuery;

  eq(value: any): boolean;

  partiallyEq(value: any): boolean;

  getDisplayName(): string;
}

export class OrFilterExpression implements FilterExpression{
  public filter_type: "OrExpression" = "OrExpression";
  public filter: TagQuery[] = [];

  constructor(tags: TagQuery[]) {
    this.filter = tags;
  }

  public eq(value: any): boolean {
    return this == value
  }

  public partiallyEq(value: any): boolean {
    return this == value;
  }

  public getDisplayName(): string {
    return this.filter.map(t => t.getNormalizedTag()).join(" OR ");
  }
}

export class SingleFilterExpression implements FilterExpression {
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
}
