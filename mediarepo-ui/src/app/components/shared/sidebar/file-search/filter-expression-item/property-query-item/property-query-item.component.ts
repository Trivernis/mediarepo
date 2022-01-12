import {Component, Input, OnChanges, OnInit, SimpleChanges} from "@angular/core";
import {PropertyQuery, ValueComparator} from "../../../../../../../api/api-types/files";

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

    private static buildExpression(property: string, comparator: string, value: string): string {
        return `.${property} ${comparator} ${value}`;
    }

    public ngOnInit(): void {
        this.stringExpression = this.getStringExpression();
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["propertyQuery"]) {
            this.stringExpression = this.getStringExpression();
        }
    }

    public getStringExpression(): string {
        if ("Status" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression("Status", "is", this.propertyQuery.Status);
        } else if ("FileSize" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression(
                "FileSize",
                this.getComparator(this.propertyQuery.FileSize),
                this.getValue(this.propertyQuery.FileSize).toString()
            );
        } else if ("ImportedTime" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression(
                "ImportedTime",
                this.getComparator(this.propertyQuery.ImportedTime),
                this.getValue(this.propertyQuery.ImportedTime).toISOString()
            );
        } else if ("ChangedTime" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression(
                "ChangedTime",
                this.getComparator(this.propertyQuery.ChangedTime),
                this.getValue(this.propertyQuery.ChangedTime).toISOString()
            );
        } else if ("CreatedTime" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression(
                "CreatedTime",
                this.getComparator(this.propertyQuery.CreatedTime),
                this.getValue(this.propertyQuery.CreatedTime).toISOString()
            );
        } else if ("TagCount" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression(
                "TagCount",
                this.getComparator(this.propertyQuery.TagCount),
                this.getValue(this.propertyQuery.TagCount).toString()
            );
        } else if ("Cd" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression("ContentDescriptor", "is", this.propertyQuery.Cd);
        } else if ("Id" in this.propertyQuery) {
            return PropertyQueryItemComponent.buildExpression("FileId", "is", this.propertyQuery.Id.toString());
        } else {
            return "Invalid Expression";
        }
    }

    public getComparator(value: ValueComparator<any>): "=" | "<" | ">" | "between" {
        if ("Greater" in value) {
            return ">";
        } else if ("Equal" in value) {
            return "=";
        } else if ("Less" in value) {
            return "<";
        } else {
            return "between";
        }
    }

    public getValue<T>(value: ValueComparator<T>): T {
        const singleValueKeys: ("Greater" | "Equal" | "Less")[] = ["Greater", "Equal", "Less"];

        for (const key of singleValueKeys) {
            if (key in value) {
                //@ts-ignore
                return value[key];
            }
        }
        if ("Between" in value) {
            return value.Between[0];
        } else {
            return "" as unknown as T;  // unreachable
        }
    }
}
