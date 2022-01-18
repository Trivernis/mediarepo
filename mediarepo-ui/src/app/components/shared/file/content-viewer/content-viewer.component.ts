import {
    AfterViewInit,
    ChangeDetectionStrategy,
    Component,
    Input,
    OnChanges,
    OnDestroy,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {SafeResourceUrl} from "@angular/platform-browser";
import {File} from "../../../../../api/models/File";
import {FileService} from "../../../../services/file/file.service";
import {FileHelper} from "../../../../services/file/file.helper";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {BusyIndicatorComponent} from "../../app-common/busy-indicator/busy-indicator.component";

type ContentType = "image" | "video" | "audio" | "other";

@Component({
    selector: "app-content-viewer",
    templateUrl: "./content-viewer.component.html",
    styleUrls: ["./content-viewer.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ContentViewerComponent implements AfterViewInit, OnChanges, OnDestroy {
    @Input() file!: File;

    public contentUrl: SafeResourceUrl | undefined;
    public blobUrl: SafeResourceUrl | undefined;
    public contentType: ContentType = "other";

    @ViewChild(BusyIndicatorComponent) busyIndicator!: BusyIndicatorComponent;

    constructor(
        private errorBroker: ErrorBrokerService,
        private fileService: FileService
    ) {
    }

    public async ngAfterViewInit() {
        this.contentType = this.getContentType();
        if (["audio", "video"].includes(this.contentType)) {
            await this.loadBlobUrl();
        } else {
            this.contentUrl = this.fileService.buildContentUrl(this.file);
        }
    }

    public async ngOnChanges(changes: SimpleChanges) {
        if (changes["file"]) {
            this.contentType = this.getContentType();

            if (["audio", "video"].includes(this.contentType) && this.busyIndicator) {
                await this.loadBlobUrl();
            } else {
                this.contentUrl = this.fileService.buildContentUrl(this.file);
                this.unloadBlobUrl();
            }
        }
    }

    public ngOnDestroy(): void {
        this.unloadBlobUrl();
    }

    public async downloadContent() {
        const path = await FileHelper.getFileDownloadLocation(this.file);

        if (path) {
            try {
                await this.fileService.saveFile(this.file, path);
            } catch (err) {
                this.errorBroker.showError(err);
            }
        }
    }

    public async loadBlobUrl(): Promise<void> {
        await this.busyIndicator.wrapAsyncOperation(async () => {
            const startId = this.file.id;
            this.unloadBlobUrl();
            const url = await this.fileService.readFile(this.file);
            if (startId === this.file.id) {
                this.blobUrl = url;
            }
        });
    }

    private getContentType(): ContentType {
        let mimeParts = this.file.mimeType.split("/");
        const type = mimeParts.shift() ?? "other";

        switch (type) {
            case "image":
                return "image";
            case "video":
                return "video";
            case "audio":
                return "audio";
            default:
                return "other";
        }
    }

    private unloadBlobUrl() {
        if (this.blobUrl) {
            URL?.revokeObjectURL(this.blobUrl as string);
            this.blobUrl = undefined;
        }
    }
}
