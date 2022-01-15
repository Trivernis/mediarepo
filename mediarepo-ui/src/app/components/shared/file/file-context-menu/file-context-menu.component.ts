import {Component, EventEmitter, Output, ViewChild} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {ContextMenuComponent} from "../../app-common/context-menu/context-menu.component";
import {clipboard} from "@tauri-apps/api";
import {FileService} from "../../../../services/file/file.service";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {FileHelper} from "../../../../services/file/file.helper";
import {FileStatus} from "../../../../../api/api-types/files";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {BusyDialogComponent} from "../../app-common/busy-dialog/busy-dialog.component";
import {BehaviorSubject} from "rxjs";

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
export class FileContextMenuComponent {

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
            const newFile = await this.fileService.updateFileStatus(this.files[0].id, status);
            this.files[0].status = newFile.status;
        } else {
            await this.iterateWithProgress(
                `Updating file status to '${status}'`,
                this.files,
                async (file) => {
                    const newFile = await this.fileService.updateFileStatus(file.id, status);
                    file.status = newFile.status;
                }
            );
        }
        this.fileUpdate.emit();
    }

    public async deletePermanently() {
        if (this.files.length === 1) {
            await this.fileService.deleteFile(this.files[0].id);
        } else {
            await this.iterateWithProgress(
                "Deleting files",
                this.files,
                async (file) => this.fileService.deleteFile(file.id)
            );
        }
        this.fileUpdate.emit();
    }

    private applyStatus() {
        this.actionDeletePermantently = true;
        for (const file of this.files) {
            this.actionDeletePermantently &&= file.status === "Deleted";
            this.actionDelete ||= file.status !== "Deleted";
            this.actionArchive ||= file.status !== "Archived";
            this.actionImported ||= file.status !== "Imported";
            this.actionRestore ||= file.status === "Deleted";
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
}
