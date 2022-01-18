import {ChangeDetectionStrategy, Component, EventEmitter, Input, OnChanges, Output, SimpleChanges} from "@angular/core";
import {FilterExpression, FilterQuery} from "../../../../../../../api/api-types/files";
import {enumerate} from "../../../../../../utils/list-utils";

@Component({
    selector: "app-filter-expression-list-item",
    templateUrl: "./filter-expression-list-item.component.html",
    styleUrls: ["./filter-expression-list-item.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
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

    private parseFilter() {
        if (this.filter && "OrExpression" in this.filter) {
            this.orExpression = enumerate(this.filter.OrExpression);
        } else if (this.filter) {
            this.query = this.filter.Query;
        }
    }
}
