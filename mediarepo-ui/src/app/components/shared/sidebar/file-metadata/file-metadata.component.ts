import {
    Component,
    Input,
    OnChanges,
    OnInit,
    SimpleChanges
} from "@angular/core";
import {File} from "../../../../models/File";
import {FileService} from "../../../../services/file/file.service";

@Component({
    selector: "app-file-metadata",
    templateUrl: "./file-metadata.component.html",
    styleUrls: ["./file-metadata.component.scss"]
})
export class FileMetadataComponent {

    @Input() file!: File;

    constructor(private fileService: FileService) {
    }

    public async saveFileName(name: string) {
        const newFile = await this.fileService.updateFileName(this.file, name);
        this.file.name = newFile.name;
    }
}
