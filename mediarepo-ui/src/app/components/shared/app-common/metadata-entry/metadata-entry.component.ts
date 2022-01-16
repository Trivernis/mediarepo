import {Component, Input} from "@angular/core";

@Component({
    selector: "app-metadata-entry",
    templateUrl: "./metadata-entry.component.html",
    styleUrls: ["./metadata-entry.component.scss"]
})
export class MetadataEntryComponent {

    @Input() attributeName!: string;
    @Input() value!: string;

    constructor() {
    }
}
