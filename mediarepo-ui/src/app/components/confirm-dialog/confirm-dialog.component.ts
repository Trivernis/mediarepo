import {Component, Inject} from '@angular/core';
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {ThemePalette} from "@angular/material/core";

@Component({
  selector: 'app-confirm-dialog',
  templateUrl: './confirm-dialog.component.html',
  styleUrls: ['./confirm-dialog.component.scss']
})
export class ConfirmDialogComponent {

  title = "";
  message = "";
  confirmAction = "";
  confirmColor: ThemePalette = "primary";
  denyAction = "Cancel";
  denyColor: ThemePalette = "accent";

  constructor(
    public dialogRef: MatDialogRef<ConfirmDialogComponent>,
    @Inject(
      MAT_DIALOG_DATA) data: { title: string, message: string, confirmAction: string, denyAction?: string, confirmColor?: ThemePalette, denyColor?: ThemePalette }
  ) {
    this.title = data.title;
    this.message = data.message;
    this.confirmAction = data.confirmAction;
    this.denyAction = data.denyAction ?? this.denyAction;
    this.confirmColor = data.confirmColor ?? this.confirmColor;
    this.denyColor = data.denyColor ?? this.denyColor;
  }

  public closeDialog(result: boolean) {
    this.dialogRef.close(result);
  }
}
