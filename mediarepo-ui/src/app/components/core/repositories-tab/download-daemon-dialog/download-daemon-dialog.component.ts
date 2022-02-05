import {Component, Inject} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {shell} from "@tauri-apps/api";

@Component({
    selector: "app-download-daemon-dialog",
    templateUrl: "./download-daemon-dialog.component.html",
    styleUrls: ["./download-daemon-dialog.component.scss"]
})
export class DownloadDaemonDialogComponent {

    constructor(
        public dialogRef: MatDialogRef<DownloadDaemonDialogComponent>,
        @Inject(
            MAT_DIALOG_DATA) data: any
    ) {
    }


    public async onClickDownloadDaemon() {
        await shell.open("https://github.com/Trivernis/mediarepo-daemon");
    }

    closeDialog(result: boolean) {
        this.dialogRef.close(result);
    }
}
