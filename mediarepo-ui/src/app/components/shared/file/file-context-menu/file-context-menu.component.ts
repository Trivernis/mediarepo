import {Component, EventEmitter, OnChanges, Output, SimpleChanges, ViewChild} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {ContextMenuComponent} from "../../app-common/context-menu/context-menu.component";
import {FileService} from "../../../../services/file/file.service";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {BusyDialogComponent} from "../../app-common/busy-dialog/busy-dialog.component";
import {BehaviorSubject} from "rxjs";
import {FileActionBaseComponent} from "../../app-base/file-action-base/file-action-base.component";

type ProgressDialogContext = {
    dialog: MatDialogRef<BusyDialogComponent>,
    progress: BehaviorSubject<number>,
    message: BehaviorSubject<string>,
};

@Component({
    selector: "app-file-context-menu",
    templateUrl: "./file-context-menu.component.html",
    styleUrls: ["./file-context-menu.component.scss"]
})
export class FileContextMenuComponent extends FileActionBaseComponent implements OnChanges {

    public files: File[] = [];

    public actionImported = false;
    public actionArchive = false;
    public actionRestore = false;
    public actionDelete = false;
    public actionDeletePermantently = false;

    @ViewChild("contextMenu") contextMenu!: ContextMenuComponent;
    @Output() fileUpdate = new EventEmitter<void>();

    constructor(fileService: FileService, errorBroker: ErrorBrokerService, dialog: MatDialog) {
        super(dialog, errorBroker, fileService);
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["files"]) {
            this.applyStatus();
        }
    }

    public onContextMenu(event: MouseEvent, files: File[]) {
        this.files = files;
        this.applyStatus();
        this.contextMenu.onContextMenu(event);
    }

    private applyStatus() {
        this.actionDeletePermantently = true;
        this.actionDelete = this.actionArchive = this.actionImported = this.actionRestore = false;

        for (const file of this.files) {
            this.actionDeletePermantently &&= file.status === "Deleted";
            this.actionDelete ||= file.status !== "Deleted";
            this.actionArchive ||= file.status !== "Archived" && file.status !== "Deleted";
            this.actionImported ||= file.status !== "Imported" && file.status !== "Deleted";
            this.actionRestore ||= file.status === "Deleted";
        }
    }
}
