import {ChangeDetectionStrategy, Component, Input} from "@angular/core";

@Component({
    selector: "app-metadata-entry",
    templateUrl: "./metadata-entry.component.html",
    styleUrls: ["./metadata-entry.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class MetadataEntryComponent {

    @Input() display: "multiline" | "line" = "multiline";
    @Input() attributeName!: string;

    constructor() {
    }
}
