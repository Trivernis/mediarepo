import {ChangeDetectionStrategy, Component, Input, OnChanges, OnInit, SimpleChanges, ViewChild} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {FileService} from "../../../../services/file/file.service";
import {FileMetadata} from "../../../../../api/api-types/files";
import {BusyIndicatorComponent} from "../../app-common/busy-indicator/busy-indicator.component";

@Component({
    selector: "app-file-metadata",
    templateUrl: "./file-metadata.component.html",
    styleUrls: ["./file-metadata.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FileMetadataComponent implements OnInit, OnChanges {

    @Input() file!: File;
    public fileMetadata: FileMetadata | undefined;

    @ViewChild(BusyIndicatorComponent) busyIndicator!: BusyIndicatorComponent;

    constructor(private fileService: FileService) {
    }

    public async ngOnInit() {
        await this.busyIndicator.wrapAsyncOperation(async () => {
            this.fileMetadata = await this.fileService.getFileMetadata(this.file.id);
        });
    }

    public async ngOnChanges(changes: SimpleChanges) {
        if (changes["file"] && (!this.fileMetadata || this.fileMetadata.file_id != this.file.id)) {
            await this.busyIndicator.wrapAsyncOperation(async () => {
                this.fileMetadata = await this.fileService.getFileMetadata(this.file.id);
            });
        }
    }

    public async saveFileName(name: string) {
        await this.busyIndicator.wrapAsyncOperation(async () => {
            const newFile = await this.fileService.updateFileName(this.file.id, name);
            if (this.fileMetadata) {
                this.fileMetadata.name = newFile.name;
            }
        });
    }
}
