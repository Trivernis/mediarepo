import {Component, Input, OnInit} from "@angular/core";
import {File} from "../../../models/File";
import {TabState} from "../../../models/TabState";

@Component({
    selector: "app-import-tab",
    templateUrl: "./import-tab.component.html",
    styleUrls: ["./import-tab.component.scss"]
})
export class ImportTabComponent implements OnInit {

    @Input() state!: TabState;

    public files: File[] = [];
    public selectedFiles: File[] = [];

    private newFiles: File[] = [];

    constructor() {
    }

    public ngOnInit(): void {
        this.state.files.subscribe(files => files? this.files = files : undefined);
    }

    /**
     * Adds an imported file to the list of imported files
     * @param {File} file
     * @returns {Promise<void>}
     */
    public async addFileFromImport(file: File) {
        this.newFiles.push(file);
        if (this.newFiles.length % 50 === 0) {  // refresh every 50 pictures
            this.refreshFileView();
        }
    }

    /**
     * Refreshes the file view
     * @returns {Promise<void>}
     */
    public refreshFileView() {
        this.state.files.next([...this.state.files.value, ...this.newFiles]);
        this.newFiles = [];
    }

    public onFileSelect(files: File[]) {
        this.selectedFiles = files;
        if (files.length === 1) {
            this.state.selectedFileHash.next(files[0].hash);
        } else {
            this.state.selectedFileHash.next(undefined);
        }
    }

    public getSelectedFileFromState(): File | undefined {
        const selectedHash = this.state.selectedFileHash.value;

        if (selectedHash && this.files) {
            return this.files.find(f => f.hash === selectedHash);
        } else {
            return undefined;
        }
    }
}
