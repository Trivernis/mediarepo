import {ChangeDetectionStrategy, Component, EventEmitter, Output} from "@angular/core";
import {File} from "../../../../../api/models/File";

@Component({
    selector: "app-file-import",
    templateUrl: "./file-import.component.html",
    styleUrls: ["./file-import.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FileImportComponent {

    @Output() fileImported = new EventEmitter<File>();
    @Output() importFinished = new EventEmitter<void>();

    constructor() {
    }
}
