import {
    AfterViewChecked,
    AfterViewInit,
    ChangeDetectionStrategy,
    ChangeDetectorRef,
    Component,
    Input,
    OnChanges,
    SimpleChanges
} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {FileService} from "../../../../services/file/file.service";
import {FileHelper} from "../../../../services/file/file.helper";
import {SafeResourceUrl} from "@angular/platform-browser";
import {BehaviorSubject} from "rxjs";

@Component({
    selector: "app-file-thumbnail",
    templateUrl: "./file-thumbnail.component.html",
    styleUrls: ["./file-thumbnail.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FileThumbnailComponent implements OnChanges, AfterViewInit, AfterViewChecked {

    @Input() file!: File;
    @Input() public fileChanged: BehaviorSubject<void> = new BehaviorSubject<void>(undefined);

    public thumbUrl: SafeResourceUrl | undefined;
    public fileType!: string;
    public thumbnailSupported: boolean = false;

    private supportedThumbnailTypes = ["image", "video"];
    private previousStatus = "imported";

    constructor(private changeDetector: ChangeDetectorRef, private fileService: FileService) {
    }

    public async ngAfterViewInit() {
        if (this.thumbnailSupported) {
            this.thumbUrl = this.fileService.buildThumbnailUrl(this.file, 250, 250);
        }
    }

    public ngAfterViewChecked(): void {
        if (this.file && this.file.status != this.previousStatus) {
            this.previousStatus = this.file.status;
            this.changeDetector.markForCheck();
        }
    }

    public async ngOnChanges(changes: SimpleChanges) {
        if (changes["file"]) {
            this.thumbUrl = this.fileService.buildThumbnailUrl(this.file,
                250, 250
            );
            this.fileType = this.getFileType();
            this.thumbnailSupported = this.getThumbnailSupported();
        }
        if (changes["fileChanged"]) {
            this.fileChanged.subscribe(() => this.changeDetector.markForCheck());
        }
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
