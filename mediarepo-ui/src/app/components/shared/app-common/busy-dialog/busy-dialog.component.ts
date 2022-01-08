import {Component, Inject} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {BehaviorSubject} from "rxjs";

export type BusyDialogData = {
    title: string,
    message: BehaviorSubject<string>,
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

    constructor(public dialogRef: MatDialogRef<BusyDialogComponent>, @Inject(MAT_DIALOG_DATA) data: BusyDialogData) {
        this.title = data.title;
        data.message.subscribe(m => this.message = m);
        this.allowCancel = data.allowCancel ?? false;
    }
}
