import {Component, ViewChild} from "@angular/core";
import {File} from "../../../../models/File";
import {
    ContextMenuComponent
} from "../../app-common/context-menu/context-menu.component";
import {clipboard} from "@tauri-apps/api";
import {FileService} from "../../../../services/file/file.service";
import {
    ErrorBrokerService
} from "../../../../services/error-broker/error-broker.service";
import {FileHelper} from "../../../../services/file/file.helper";

@Component({
    selector: "app-file-context-menu",
    templateUrl: "./file-context-menu.component.html",
    styleUrls: ["./file-context-menu.component.scss"]
})
export class FileContextMenuComponent {

    public file!: File;

    @ViewChild("contextMenu") contextMenu!: ContextMenuComponent;

    constructor(private fileService: FileService, private errorBroker: ErrorBrokerService) {
    }

    public onContextMenu(event: MouseEvent, file: File) {
        this.file = file;
        this.contextMenu.onContextMenu(event);
    }

    public async copyFileHash(): Promise<void> {
        await clipboard.writeText(this.file.hash);
    }

    public async exportFile(): Promise<void> {
        const path = await FileHelper.getFileDownloadLocation(this.file)

        if (path) {
            try {
                await this.fileService.saveFile(this.file, path);
            } catch (err) {
                this.errorBroker.showError(err);
            }
        }
    }
}
