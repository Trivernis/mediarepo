import {
    Component,
    Input,
    OnChanges,
    OnInit,
    SimpleChanges
} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {FileService} from "../../../../services/file/file.service";
import {FileMetadata} from "../../../../../api/api-types/files";

@Component({
    selector: "app-file-metadata",
    templateUrl: "./file-metadata.component.html",
    styleUrls: ["./file-metadata.component.scss"]
})
export class FileMetadataComponent implements OnInit, OnChanges {

    @Input() file!: File;
    public fileMetadata: FileMetadata | undefined;
    public loading = false;

    constructor(private fileService: FileService) {
    }

    public async ngOnInit() {
        this.loading = true;
        this.fileMetadata = await this.fileService.getFileMetadata(this.file.id);
        this.loading = false;
    }

    public async ngOnChanges(changes:SimpleChanges) {
        if (changes["file"] && (!this.fileMetadata || this.fileMetadata.file_id != this.file.id)) {
            this.loading = true;
            this.fileMetadata = await this.fileService.getFileMetadata(this.file.id);
            this.loading = false;
        }
    }

    public async saveFileName(name: string) {
        this.loading = true;
        const newFile = await this.fileService.updateFileName(this.file.id, name);
        if (this.fileMetadata) {
            this.fileMetadata.name = newFile.name;
        }
        this.loading = false;
    }
}
