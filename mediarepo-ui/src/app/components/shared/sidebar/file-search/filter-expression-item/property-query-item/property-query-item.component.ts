import {Component, Input, OnChanges, OnInit, SimpleChanges} from "@angular/core";
import {PropertyQuery} from "../../../../../../../api/api-types/files";
import {propertyQueryToString} from "../../../../../../utils/filter-utils";

@Component({
    selector: "app-property-query-item",
    templateUrl: "./property-query-item.component.html",
    styleUrls: ["./property-query-item.component.scss"]
})
export class PropertyQueryItemComponent implements OnInit, OnChanges {

    @Input() propertyQuery!: PropertyQuery;

    public stringExpression: string = "No Expression";

    constructor() {
    }

    public ngOnInit(): void {
        this.stringExpression = propertyQueryToString(this.propertyQuery);
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["propertyQuery"]) {
            this.stringExpression = propertyQueryToString(this.propertyQuery);
        }
    }
}
