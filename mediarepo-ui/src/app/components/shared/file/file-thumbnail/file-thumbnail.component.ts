import {
    AfterViewInit,
    ChangeDetectionStrategy,
    ChangeDetectorRef,
    Component,
    EventEmitter,
    Input,
    OnChanges,
    Output,
    SimpleChanges
} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {FileService} from "../../../../services/file/file.service";
import {FileHelper} from "../../../../services/file/file.helper";
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
    selector: "app-file-thumbnail",
    templateUrl: "./file-thumbnail.component.html",
    styleUrls: ["./file-thumbnail.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FileThumbnailComponent implements OnChanges, AfterViewInit {

    @Input() file!: File;
    @Output() loadEnd = new EventEmitter<void>();

    public thumbUrl: SafeResourceUrl | undefined;
    public fileType!: string;
    public thumbnailSupported: boolean = false;
    public displayError = false;

    private supportedThumbnailTypes = ["image", "video"];

    constructor(private changeDetector: ChangeDetectorRef, private fileService: FileService) {
    }

    public async ngAfterViewInit() {
        if (this.thumbnailSupported) {
            this.thumbUrl = this.fileService.buildThumbnailUrl(this.file, 250, 250);
        }
    }

    public async ngOnChanges(changes: SimpleChanges) {
        if (changes["file"]) {
            this.thumbUrl = this.fileService.buildThumbnailUrl(this.file,
                250, 250
            );
            this.fileType = this.getFileType();
            this.thumbnailSupported = this.getThumbnailSupported();
            this.displayError = false;
        }
    }

    public onImageLoadError(): void {
        this.loadEnd.emit();
        this.displayError = true;
    }

    private getThumbnailSupported(): boolean {
        const mimeParts = FileHelper.parseMime(this.file.mimeType);

        return !!mimeParts && this.supportedThumbnailTypes.includes(
            mimeParts[0]);
    }

    private getFileType(): string {
        const mimeParts = FileHelper.parseMime(this.file.mimeType);
        return (mimeParts && mimeParts[0]) ?? "other";
    }
}
