import {ChangeDetectionStrategy, Component, Input, OnChanges, OnInit, SimpleChanges} from "@angular/core";
import {FilterExpression, FilterQuery} from "../../../../../../api/api-types/files";

@Component({
    selector: "app-filter-expression-item",
    templateUrl: "./filter-expression-item.component.html",
    styleUrls: ["./filter-expression-item.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FilterExpressionItemComponent implements OnInit, OnChanges {
    @Input() filter!: FilterExpression;
    public orExpression?: FilterQuery[];
    public query?: FilterQuery;

    constructor() {
    }

    public ngOnInit(): void {
        this.parseQuery();
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["filter"]) {
            this.parseQuery();
        }
    }

    private parseQuery() {
        if ("Query" in this.filter) {
            this.query = this.filter.Query;
        } else {
            this.orExpression = this.filter.OrExpression;
        }
    }
}
