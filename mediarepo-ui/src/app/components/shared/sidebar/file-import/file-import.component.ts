import {ChangeDetectionStrategy, Component, Input} from "@angular/core";
import {ImportTabState} from "../../../../models/state/ImportTabState";

@Component({
    selector: "app-file-import",
    templateUrl: "./file-import.component.html",
    styleUrls: ["./file-import.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FileImportComponent {

    @Input() state!: ImportTabState;

    constructor() {
    }
}
