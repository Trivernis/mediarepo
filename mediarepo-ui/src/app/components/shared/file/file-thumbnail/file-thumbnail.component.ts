import {
    Component,
    Input,
    OnChanges,
    OnInit,
    SimpleChanges
} from "@angular/core";
import {File} from "../../../../models/File";
import {FileService} from "../../../../services/file/file.service";
import {FileHelper} from "../../../../services/file/file.helper";
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
    selector: "app-file-thumbnail",
    templateUrl: "./file-thumbnail.component.html",
    styleUrls: ["./file-thumbnail.component.scss"]
})
export class FileThumbnailComponent implements OnInit, OnChanges {

    @Input() file!: File;

    public thumbUrl: SafeResourceUrl | undefined;

    private supportedThumbnailTypes = ["image", "video"]

    constructor(private fileService: FileService) {
    }

    public ngOnInit(): void {
        this.thumbUrl = this.fileService.buildThumbnailUrl(this.file, 250, 250);
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["file"]) {
            this.thumbUrl = this.fileService.buildThumbnailUrl(this.file, 250,
                250)
        }
    }

    public getThumbnailSupported(): boolean {
        const mimeParts = FileHelper.parseMime(this.file.mime_type);

        return !!mimeParts && this.supportedThumbnailTypes.includes(
            mimeParts[0]);
    }

    public getFileType(): string {
        const mimeParts = FileHelper.parseMime(this.file.mime_type);
        return (mimeParts && mimeParts[0]) ?? "other";
    }
}
