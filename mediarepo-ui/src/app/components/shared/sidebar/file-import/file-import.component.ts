import {Component, EventEmitter, Output} from "@angular/core";
import {File} from "../../../../models/File";

@Component({
    selector: "app-file-import",
    templateUrl: "./file-import.component.html",
    styleUrls: ["./file-import.component.scss"]
})
export class FileImportComponent {

    @Output() fileImported = new EventEmitter<File>();
    @Output() importFinished = new EventEmitter<void>();

    constructor() {
    }
}
