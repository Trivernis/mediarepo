import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {ImportTabState} from "../../../../models/state/ImportTabState";

@Component({
    selector: "app-import-tab-sidebar",
    templateUrl: "./import-tab-sidebar.component.html",
    styleUrls: ["./import-tab-sidebar.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ImportTabSidebarComponent {

    @Input() state!: ImportTabState;
    @Input() selectedFiles: File[] = [];
    @Output() fileImported = new EventEmitter<File>();
    @Output() importFinished = new EventEmitter<void>();

    constructor() {
    }
}
