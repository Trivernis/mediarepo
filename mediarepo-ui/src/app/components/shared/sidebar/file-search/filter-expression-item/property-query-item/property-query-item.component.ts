import {Component, Input, OnChanges, OnInit, SimpleChanges} from "@angular/core";
import {PropertyQuery} from "../../../../../../../api/api-types/files";
import {propertyQueryToStringParts} from "../../../../../../utils/filter-utils";

@Component({
    selector: "app-property-query-item",
    templateUrl: "./property-query-item.component.html",
    styleUrls: ["./property-query-item.component.scss"]
})
export class PropertyQueryItemComponent implements OnInit, OnChanges {

    @Input() propertyQuery!: PropertyQuery;

    public propertyName: string = "No Property";
    public comparator: string = "!!";
    public value: string = "null";

    constructor() {
    }

    public ngOnInit(): void {
        [this.propertyName, this.comparator, this.value] = propertyQueryToStringParts(this.propertyQuery);
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["propertyQuery"]) {
            [this.propertyName, this.comparator, this.value] = propertyQueryToStringParts(this.propertyQuery);
        }
    }
}
