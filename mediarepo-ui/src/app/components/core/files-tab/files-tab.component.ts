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
    }
}
