import {Component, EventEmitter, OnChanges, Output, SimpleChanges, ViewChild} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {ContextMenuComponent} from "../../app-common/context-menu/context-menu.component";
import {clipboard} from "@tauri-apps/api";
import {FileService} from "../../../../services/file/file.service";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {FileHelper} from "../../../../services/file/file.helper";
import {FileStatus} from "../../../../../api/api-types/files";
import {MatDialog, MatDialogConfig, MatDialogRef} from "@angular/material/dialog";
import {BusyDialogComponent} from "../../app-common/busy-dialog/busy-dialog.component";
import {BehaviorSubject} from "rxjs";
import {ConfirmDialogComponent, ConfirmDialogData} from "../../app-common/confirm-dialog/confirm-dialog.component";
import {SafeResourceUrl} from "@angular/platform-browser";

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
export class FileContextMenuComponent implements OnChanges {

    public files: File[] = [];

    public actionImported = false;
    public actionArchive = false;
    public actionRestore = false;
    public actionDelete = false;
    public actionDeletePermantently = false;

    @ViewChild("contextMenu") contextMenu!: ContextMenuComponent;
    @Output() fileUpdate = new EventEmitter<void>();

    constructor(private fileService: FileService, private errorBroker: ErrorBrokerService, private dialog: MatDialog) {
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

    public async copyFileHash(): Promise<void> {
        await clipboard.writeText(this.files[0].cd);
    }

    public async exportFile(): Promise<void> {
        const path = await FileHelper.getFileDownloadLocation(this.files[0]);

        if (path) {
            try {
                await this.fileService.saveFile(this.files[0], path);
            } catch (err) {
                this.errorBroker.showError(err);
            }
        }
    }

    public async updateStatus(status: FileStatus) {
        if (this.files.length === 1) {
            let changeConfirmed;

            if (status === "Deleted") {
                changeConfirmed = await this.openConfirmDialog(
                    "Confirm deletion",
                    "Do you really want to move this file to trash?",
                    "Delete",
                    "warn",
                    this.getImageThumbnail(this.files[0])
                );
            } else {
                changeConfirmed = true;
            }

            if (changeConfirmed) {
                const newFile = await this.fileService.updateFileStatus(this.files[0].id, status);
                this.files[0].status = newFile.status;
                this.fileUpdate.emit();
                this.applyStatus();
            }
        } else {
            const statusChangeConfirmed = await this.openConfirmDialog(
                "Confirm mass status change",
                `Do you really want to change the status of ${this.files.length} files to '${status}'?`,
                "Change status",
                status === "Deleted" ? "warn" : "primary"
            );
            if (statusChangeConfirmed) {
                await this.iterateWithProgress(
                    `Updating file status to '${status}'`,
                    this.files,
                    async (file) => {
                        const newFile = await this.fileService.updateFileStatus(file.id, status);
                        file.status = newFile.status;
                    }
                );
                this.fileUpdate.emit();
                this.applyStatus();
            }
        }
    }

    public async deletePermanently() {
        if (this.files.length === 1) {
            const deletionConfirmed = await this.openConfirmDialog(
                "Confirm deletion",
                "Do you really want to permanently delete this file?",
                "Delete permanently",
                "warn",
                this.getImageThumbnail(this.files[0]),
            );
            if (deletionConfirmed) {
                await this.fileService.deleteFile(this.files[0].id);
                this.fileUpdate.emit();
                this.applyStatus();
            }
        } else {
            const deletionConfirmed = await this.openConfirmDialog(
                "Confirm mass deletion",
                `Do you really want to permanently delete ${this.files.length} files?`,
                "Delete permanently",
                "warn"
            );
            if (deletionConfirmed) {
                await this.iterateWithProgress(
                    "Deleting files",
                    this.files,
                    async (file) => this.fileService.deleteFile(file.id)
                );
                this.fileUpdate.emit();
                this.applyStatus();
            }
        }
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

    private getImageThumbnail(file: File): SafeResourceUrl | undefined {
        const mimeParts = FileHelper.parseMime(file.mimeType);

        if (mimeParts && ["image", "video"].includes(mimeParts[0])) {
            return this.fileService.buildThumbnailUrl(file, 250, 250);
        } else {
            return;
        }
    }

    private async iterateWithProgress<T>(title: string, items: T[], action: (arg: T) => Promise<any>): Promise<void> {
        const totalCount = items.length;
        const dialogCtx = this.openProgressDialog(title, `0/${totalCount}`);
        let count = 0;

        for (const item of items) {
            await action(item);
            dialogCtx.message.next(`${++count}/${totalCount}`);
            dialogCtx.progress.next(count / totalCount);
        }
        dialogCtx.dialog.close(true);
    }

    private openProgressDialog(title: string, message: string): ProgressDialogContext {
        const dialogMessage = new BehaviorSubject(message);
        const dialogProgress = new BehaviorSubject(0);

        const dialog = this.dialog.open(BusyDialogComponent, {
            data: {
                message: dialogMessage,
                progress: dialogProgress,
                title,
                allowCancel: false,
            },
            disableClose: true,
            minWidth: "30%",
            minHeight: "30%",
        });

        return {
            dialog,
            message: dialogMessage,
            progress: dialogProgress,
        };
    }

    private openConfirmDialog(
        title: string,
        question: string,
        confirmAction: string,
        confirmColor?: "primary" | "warn",
        image?: SafeResourceUrl | string
    ): Promise<boolean> {
        const dialog = this.dialog.open(ConfirmDialogComponent, {
            data: {
                title,
                message: question,
                confirmAction,
                denyAction: "Cancel",
                confirmColor,
                image
            }
        } as MatDialogConfig & { data: ConfirmDialogData });
        return dialog.afterClosed().toPromise();
    }
}
