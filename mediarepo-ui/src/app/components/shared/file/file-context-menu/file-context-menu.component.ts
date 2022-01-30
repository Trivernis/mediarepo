import {Component, EventEmitter, OnChanges, Output, SimpleChanges, ViewChild} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {ContextMenuComponent} from "../../app-common/context-menu/context-menu.component";
import {FileService} from "../../../../services/file/file.service";
import {LoggingService} from "../../../../services/logging/logging.service";
import {MatDialog} from "@angular/material/dialog";
import {FileActionBaseComponent} from "../../app-base/file-action-base/file-action-base.component";
import {FileStatus} from "../../../../../api/api-types/files";

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
    @Output() fileDeleted = new EventEmitter<File[]>();
    @Output() fileStatusChange = new EventEmitter<File[]>();

    constructor(fileService: FileService, errorBroker: LoggingService, dialog: MatDialog) {
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

    public async deleteFilesPermanently() {
        const deleted = await this.deletePermanently(this.files);

        if (deleted) {
            this.fileDeleted.emit(this.files);
        }
    }

    public async changeFileStatus(status: FileStatus) {
        await this.updateStatus(this.files, status);
        this.fileStatusChange.emit(this.files);
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
