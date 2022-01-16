import {Component, Input} from "@angular/core";
import {
    FilterExpression,
    FilterExpressionOrExpression,
    FilterExpressionQuery,
    FilterQuery,
    FilterQueryProperty,
    FilterQueryTag
} from "../../../../../../api/api-types/files";

@Component({
    selector: "app-filter-expression-item",
    templateUrl: "./filter-expression-item.component.html",
    styleUrls: ["./filter-expression-item.component.scss"]
})
export class FilterExpressionItemComponent {


    @Input() filter!: FilterExpression;

    constructor() {
    }

    public is(key: "OrExpression" | "Query"): boolean {
        return key in this.filter;
    }

    public orExpression(): FilterExpressionOrExpression {
        return this.filter as FilterExpressionOrExpression;
    }

    public query(): FilterExpressionQuery {
        return this.filter as FilterExpressionQuery;
    }

    public queryIs(query: FilterQuery, key: "Property" | "Tag"): boolean {
        return key in query;
    }

    public propertyQuery(query: FilterQuery): FilterQueryProperty {
        return query as FilterQueryProperty;
    }

    public tagQuery(query: FilterQuery): FilterQueryTag {
        return query as FilterQueryTag;
    }
}
