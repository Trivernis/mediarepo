import {ChangeDetectionStrategy, ChangeDetectorRef, Component, Input, OnInit} from "@angular/core";
import {File} from "../../../../api/models/File";
import {TabState} from "../../../models/TabState";

@Component({
    selector: "app-import-tab",
    templateUrl: "./import-tab.component.html",
    styleUrls: ["./import-tab.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ImportTabComponent implements OnInit {

    @Input() state!: TabState;

    public files: File[] = [];
    public selectedFiles: File[] = [];

    private newFiles: File[] = [];

    constructor(private changeDetector: ChangeDetectorRef) {
    }

    public ngOnInit(): void {
        this.state.files.subscribe(files => files ? this.files = files : undefined);
    }

    /**
     * Adds an imported file to the list of imported files
     * @param {File} file
     * @returns {Promise<void>}
     */
    public async addFileFromImport(file: File) {
        this.state.files.next([...this.state.files.value, file]);
        this.changeDetector.markForCheck();
    }

    public onFileSelect(files: File[]) {
        this.selectedFiles = files;
        if (files.length === 1) {
            this.state.selectedCD.next(files[0].cd);
        } else {
            this.state.selectedCD.next(undefined);
        }
    }

    public getSelectedFileFromState(): File | undefined {
        const selectedHash = this.state.selectedCD.value;

        if (selectedHash && this.files) {
            return this.files.find(f => f.cd === selectedHash);
        } else {
            return undefined;
        }
    }

    public refreshFileView(): void {
        this.changeDetector.markForCheck();
    }
}
