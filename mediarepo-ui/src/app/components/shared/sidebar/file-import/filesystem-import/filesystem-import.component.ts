import {ChangeDetectionStrategy, ChangeDetectorRef, Component, EventEmitter, Output} from "@angular/core";
import {ImportService} from "../../../../../services/import/import.service";
import {LoggingService} from "../../../../../services/logging/logging.service";
import {AddFileOptions} from "../../../../../models/AddFileOptions";
import {File} from "../../../../../../api/models/File";
import {DialogFilter} from "@tauri-apps/api/dialog";
import {FileOsMetadata} from "../../../../../../api/api-types/files";

@Component({
    selector: "app-filesystem-import",
    templateUrl: "./filesystem-import.component.html",
    styleUrls: ["./filesystem-import.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FilesystemImportComponent {

    @Output() fileImported = new EventEmitter<File>();
    @Output() importFinished = new EventEmitter<void>();

    public fileCount: number = 0;
    public files: FileOsMetadata[] = [];
    public importOptions = new AddFileOptions();
    public filters: DialogFilter[] = [
        {
            name: "Images",
            extensions: ["png", "jpg", "jpeg", "webp", "bmp", "gif"]
        },
        { name: "Videos", extensions: ["mp4", "mkv", "wmv", "avi", "webm"] },
        { name: "Audio", extensions: ["mp3", "ogg", "wav", "flac", "aac"] },
        { name: "Documents", extensions: ["pdf", "doc", "docx", "odf"] },
        { name: "Text", extensions: ["txt", "md"] },
        { name: "All", extensions: ["*"] }
    ];


    public resolving = false;
    public importing = false;
    public importingProgress = 0;
    public importingProgressTotal = 0;

    constructor(
        private changeDetector: ChangeDetectorRef,
        private errorBroker: LoggingService,
        private importService: ImportService
    ) {
    }

    public async setSelectedPaths(paths: string[]) {
        this.changeDetector.markForCheck();
        this.resolving = true;
        try {
            this.files = await this.importService.resolvePathsToFiles(paths);
            this.fileCount = this.files.length;
        } catch (err: any) {
            console.log(err);
            this.errorBroker.error(err);
        }
        this.resolving = false;
        this.changeDetector.markForCheck();
    }

    public async import() {
        this.importing = true;

        this.importingProgress = 0;
        let count = 0;

        for (const file of this.files) {
            try {
                const resultFile = await this.importService.addLocalFile(
                    file,
                    this.importOptions
                );
                this.fileImported.emit(resultFile);
            } catch (err: any) {
                console.log(err);
                this.errorBroker.error(err);
            }
            count++;
            this.importingProgress = (count / this.fileCount) * 100;
            this.importingProgressTotal = count;
        }

        this.importing = false;
        this.importFinished.emit();
    }
}
