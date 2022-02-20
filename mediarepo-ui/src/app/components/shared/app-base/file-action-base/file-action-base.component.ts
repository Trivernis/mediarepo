import {Component} from "@angular/core";
import {FileService} from "../../../../services/file/file.service";
import {clipboard} from "@tauri-apps/api";
import {FileHelper} from "../../../../services/file/file.helper";
import {FileStatus} from "../../../../../api/api-types/files";
import {File} from "../../../../../api/models/File";
import {SafeResourceUrl} from "@angular/platform-browser";
import {BehaviorSubject} from "rxjs";
import {BusyDialogComponent} from "../../app-common/busy-dialog/busy-dialog.component";
import {ConfirmDialogComponent, ConfirmDialogData} from "../../app-common/confirm-dialog/confirm-dialog.component";
import {MatDialog, MatDialogConfig, MatDialogRef} from "@angular/material/dialog";
import {LoggingService} from "../../../../services/logging/logging.service";

type ProgressDialogContext = {
    dialog: MatDialogRef<BusyDialogComponent>,
    progress: BehaviorSubject<number>,
    message: BehaviorSubject<string>,
};

@Component({
    selector: "app-file-action-base",
    template: "<h1>Do not use</h1>",
})
export class FileActionBaseComponent {
    constructor(private dialog: MatDialog, private errorBroker: LoggingService, private fileService: FileService) {
    }

    public async copyFileContentDescriptor(file: File): Promise<void> {
        await clipboard.writeText(file.cd);
    }

    public async exportFile(file: File): Promise<void> {
        const path = await FileHelper.getFileDownloadLocation(file);

        if (path) {
            await this.errorBroker.try(() => this.fileService.saveFile(file, path));
        }
    }

    public async updateStatus(files: File[], status: FileStatus) {
        if (files.length === 1) {
            let changeConfirmed;

            if (status === "Deleted") {
                changeConfirmed = await this.openConfirmDialog(
                    "Confirm deletion",
                    "Do you really want to move this file to trash?",
                    "Delete",
                    "warn",
                    this.getImageThumbnail(files[0])
                );
            } else {
                changeConfirmed = true;
            }

            if (changeConfirmed) {
                await this.errorBroker.try(async () => {
                    const newFile = await this.fileService.updateFileStatus(files[0].id, status);
                    files[0].setStatus(newFile.getStatus());
                });
            }
        } else {
            const statusChangeConfirmed = await this.openConfirmDialog(
                "Confirm mass status change",
                `Do you really want to change the status of ${files.length} files to '${status}'?`,
                "Change status",
                status === "Deleted" ? "warn" : "primary"
            );
            if (statusChangeConfirmed) {
                await this.iterateWithProgress(
                    `Updating file status to '${status}'`,
                    files,
                    (file) => this.errorBroker.try(async () => {
                        const newFile = await this.fileService.updateFileStatus(file.id, status);
                        file.setStatus(newFile.getStatus());
                    })
                );
            }
        }
    }

    public async deletePermanently(files: File[]): Promise<boolean> {
        if (files.length === 1) {
            const deletionConfirmed = await this.openConfirmDialog(
                "Confirm deletion",
                "Do you really want to permanently delete this file?",
                "Delete permanently",
                "warn",
                this.getImageThumbnail(files[0]),
            );
            if (deletionConfirmed) {
                await this.errorBroker.try(() => this.fileService.deleteFile(files[0].id));
                return true;
            }
        } else {
            const deletionConfirmed = await this.openConfirmDialog(
                "Confirm mass deletion",
                `Do you really want to permanently delete ${files.length} files?`,
                "Delete permanently",
                "warn"
            );
            if (deletionConfirmed) {
                await this.iterateWithProgress(
                    "Deleting files",
                    files,
                    (file) => this.errorBroker.try(() => this.fileService.deleteFile(file.id))
                );
                return true;
            }
        }
        return false;
    }

    protected getImageThumbnail(file: File): SafeResourceUrl | undefined {
        const mimeParts = FileHelper.parseMime(file.mimeType);

        if (mimeParts && ["image", "video"].includes(mimeParts[0])) {
            return this.fileService.buildThumbnailUrl(file, 250, 250);
        } else {
            return;
        }
    }

    protected async iterateWithProgress<T>(title: string, items: T[], action: (arg: T) => Promise<any>): Promise<void> {
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

    protected openProgressDialog(title: string, message: string): ProgressDialogContext {
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

    protected openConfirmDialog(
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
