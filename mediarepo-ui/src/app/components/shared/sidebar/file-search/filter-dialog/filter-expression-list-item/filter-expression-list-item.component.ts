import {Component, EventEmitter, Input, OnChanges, Output, SimpleChanges} from "@angular/core";
import {
    FilterExpression,
    FilterQuery,
    FilterQueryProperty,
    FilterQueryTag
} from "../../../../../../../api/api-types/files";
import {enumerate} from "../../../../../../utils/list-utils";

@Component({
    selector: "app-filter-expression-list-item",
    templateUrl: "./filter-expression-list-item.component.html",
    styleUrls: ["./filter-expression-list-item.component.scss"]
})
export class FilterExpressionListItemComponent implements OnChanges {


    @Input() filter!: FilterExpression;
    @Output() entrySelect = new EventEmitter<[number, FilterQuery]>();
    @Output() entryUnselect = new EventEmitter<[number, FilterQuery]>();

    @Output() appSelect = new EventEmitter<FilterQuery>();
    @Output() appUnselect = new EventEmitter<FilterQuery>();

    public orExpression: undefined | [number, FilterQuery][] = undefined;
    public query: undefined | FilterQuery = undefined;

    constructor() {
        this.parseFilter();
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["filter"]) {
            this.parseFilter();
        }
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

    private parseFilter() {
        if (this.filter && "OrExpression" in this.filter) {
            this.orExpression = enumerate(this.filter.OrExpression);
        } else if (this.filter) {
            this.query = this.filter.Query;
        }
    }
}
