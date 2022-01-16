import {Component, Input} from "@angular/core";
import {TagQuery} from "../../../../../../../api/api-types/files";

@Component({
    selector: "app-tag-query-item",
    templateUrl: "./tag-query-item.component.html",
    styleUrls: ["./tag-query-item.component.scss"]
})
export class TagQueryItemComponent {

    @Input() tagQuery!: TagQuery;

    constructor() {
    }
}
