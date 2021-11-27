import {Component, EventEmitter, Input, Output} from "@angular/core";
import {File} from "../../../../models/File";

@Component({
    selector: "app-import-tab-sidebar",
    templateUrl: "./import-tab-sidebar.component.html",
    styleUrls: ["./import-tab-sidebar.component.scss"]
})
export class ImportTabSidebarComponent {

    @Input() selectedFiles: File[] = [];
    @Output() fileImported = new EventEmitter<File>();
    @Output() importFinished = new EventEmitter<void>();

    constructor() {
    }
}
