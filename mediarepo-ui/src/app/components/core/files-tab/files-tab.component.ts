import {Component, Input, OnInit} from "@angular/core";
import {File} from "../../../models/File";
import {TabState} from "../../../models/TabState";

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
        this.state.loading.subscribe(loading => this.contentLoading = loading);
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

    public async onKeydown(event: KeyboardEvent) {
        switch (event.key) {
            case "F5":
                await this.state.findFiles()
                break;
        }
    }
}
