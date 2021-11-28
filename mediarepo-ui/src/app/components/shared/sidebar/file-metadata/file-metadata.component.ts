import {
    Component,
    Input,
    OnChanges,
    OnInit,
    SimpleChanges
} from "@angular/core";
import {File} from "../../../../models/File";

@Component({
    selector: "app-file-metadata",
    templateUrl: "./file-metadata.component.html",
    styleUrls: ["./file-metadata.component.scss"]
})
export class FileMetadataComponent implements OnInit, OnChanges {

    @Input() file!: File;

    constructor() {
    }

    public async ngOnInit(): Promise<void> {
        await this.loadFileInformation();
    }

    public async ngOnChanges(changes: SimpleChanges): Promise<void> {
        if (changes["file"]) {
            await this.loadFileInformation();
        }
    }

    private async loadFileInformation() {

    }
}
