import {Component, Inject} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {BehaviorSubject} from "rxjs";
import {ProgressBarMode} from "@angular/material/progress-bar";

export type BusyDialogData = {
    title: string,
    message?: BehaviorSubject<string>,
    progress?: BehaviorSubject<number>,
    allowCancel?: boolean,
}

@Component({
    selector: "app-busy-dialog",
    templateUrl: "./busy-dialog.component.html",
    styleUrls: ["./busy-dialog.component.scss"]
})
export class BusyDialogComponent {

    public title: string;
    public message?: string;
    public allowCancel: boolean;
    public progress = 0;
    public mode: ProgressBarMode = "indeterminate";

    constructor(public dialogRef: MatDialogRef<BusyDialogComponent>, @Inject(MAT_DIALOG_DATA) data: BusyDialogData) {
        this.title = data.title;
        if (data.message) {
            data.message.subscribe(m => this.message = m);
        }
        if (data.progress) {
            data.progress.subscribe(p => this.progress = p);
            this.mode = "determinate";
        }
        this.allowCancel = data.allowCancel ?? false;
    }
}
