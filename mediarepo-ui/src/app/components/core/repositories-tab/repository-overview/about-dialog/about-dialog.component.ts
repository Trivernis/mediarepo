import {ChangeDetectionStrategy, Component, OnInit} from "@angular/core";
import {MatDialogRef} from "@angular/material/dialog";
import {app} from "@tauri-apps/api";
import {Subject} from "rxjs";

@Component({
    selector: "app-about-dialog",
    templateUrl: "./about-dialog.component.html",
    styleUrls: ["./about-dialog.component.scss"],
    changeDetection: ChangeDetectionStrategy.Default
})
export class AboutDialogComponent implements OnInit {

    public version = new Subject<string>();
    public tauriVersion = new Subject<string>();
    public usedLibs = [
        ["Tauri", "https://tauri.studio"],
        ["Angular", "https://angular.io/"],
        ["SeaORM", "https://www.sea-ql.org"],
        ["Tokio", "https://tokio.rs/"],
        ["bromine", "https://github.com/Trivernis/bromine"]
    ];

    constructor(public dialogRef: MatDialogRef<AboutDialogComponent>) {
    }

    public async ngOnInit() {
        this.version.next(await app.getVersion());
        this.tauriVersion.next(await app.getTauriVersion());
    }
}
