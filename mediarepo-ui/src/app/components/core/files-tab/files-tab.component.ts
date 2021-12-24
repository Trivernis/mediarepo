import {Component, Input, OnInit} from "@angular/core";
import {File} from "../../../models/File";
import {TabState} from "../../../models/TabState.rs";

@Component({
    selector: "app-files-tab",
    templateUrl: "./files-tab.component.html",
    styleUrls: ["./files-tab.component.scss"]
})
export class FilesTabComponent implements OnInit {

    @Input() state!: TabState;

    files: File[] = [];
    contentLoading = false;
    selectedFiles: File[] = [];

    constructor() {
    }

    async ngOnInit() {
        this.state.files.subscribe(files => this.files = files);
    }

    async onFileSelect(files: File[]) {
        this.selectedFiles = files;
        if (files.length === 1) {
            this.state.selectedFileHash.next(files[0].hash);
        } else {
            this.state.selectedFileHash.next(undefined);
        }
    }

    public getStateSelectedFile(): File | undefined {
        const hash = this.state.selectedFileHash.value;

        if (hash) {
            return this.files.find(f => f.hash === hash);
        } else {
            return undefined;
        }
    }
}
