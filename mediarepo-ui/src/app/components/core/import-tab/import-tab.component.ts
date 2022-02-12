import {ChangeDetectionStrategy, ChangeDetectorRef, Component, Input, OnInit} from "@angular/core";
import {File} from "../../../../api/models/File";
import {ImportTabState} from "../../../models/state/ImportTabState";

@Component({
    selector: "app-import-tab",
    templateUrl: "./import-tab.component.html",
    styleUrls: ["./import-tab.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ImportTabComponent implements OnInit {

    @Input() state!: ImportTabState;

    public files: File[] = [];
    public selectedFiles: File[] = [];

    constructor(private changeDetector: ChangeDetectorRef) {
    }

    public ngOnInit(): void {
        this.state.files.subscribe(files => files ? this.files = files : undefined);
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
}
