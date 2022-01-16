import {Component, Inject} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {ThemePalette} from "@angular/material/core";
import {SafeResourceUrl} from "@angular/platform-browser";

export type ConfirmDialogData = {
    title: string,
    message: string,
    image?: string | SafeResourceUrl,
    confirmAction: string,
    denyAction?: string,
    confirmColor?: ThemePalette,
    denyColor?: ThemePalette
};

@Component({
    selector: "app-confirm-dialog",
    templateUrl: "./confirm-dialog.component.html",
    styleUrls: ["./confirm-dialog.component.scss"]
})
export class ConfirmDialogComponent {

    public title = "";
    public message = "";
    public confirmAction = "";
    public image?: string | SafeResourceUrl;
    public confirmColor: ThemePalette = "primary";
    public denyAction = "Cancel";
    public denyColor: ThemePalette = "accent";

    constructor(
        public dialogRef: MatDialogRef<ConfirmDialogComponent>,
        @Inject(
            MAT_DIALOG_DATA) data: ConfirmDialogData
    ) {
        this.title = data.title;
        this.message = data.message;
        this.confirmAction = data.confirmAction;
        this.denyAction = data.denyAction ?? this.denyAction;
        this.confirmColor = data.confirmColor ?? this.confirmColor;
        this.denyColor = data.denyColor ?? this.denyColor;
        this.image = data.image;
    }

    public closeDialog(result: boolean) {
        this.dialogRef.close(result);
    }
}
