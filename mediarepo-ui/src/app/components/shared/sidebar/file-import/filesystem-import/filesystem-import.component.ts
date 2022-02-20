import {ChangeDetectionStrategy, ChangeDetectorRef, Component, Input, OnInit} from "@angular/core";
import {ImportService} from "../../../../../services/import/import.service";
import {LoggingService} from "../../../../../services/logging/logging.service";
import {AddFileOptions} from "../../../../../models/AddFileOptions";
import {DialogFilter} from "@tauri-apps/api/dialog";
import {FileOsMetadata} from "../../../../../../api/api-types/files";
import {ImportTabState} from "../../../../../models/state/ImportTabState";

const IMAGE_EXTENSIONS = ["png", "jpg", "jpeg", "webp", "bmp", "gif"];
const VIDEO_EXTENSIONS = ["mp4", "mkv", "wmv", "avi", "webm"];
const AUDIO_EXTENSIONS = ["mp3", "ogg", "wav", "flac", "aac"];
const DOCUMENT_EXTENSIONS = ["pdf", "doc", "docx", "odf"];
const TEXT_EXTENSIONS = ["txt", "md"];

@Component({
    selector: "app-filesystem-import",
    templateUrl: "./filesystem-import.component.html",
    styleUrls: ["./filesystem-import.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FilesystemImportComponent implements OnInit {

    @Input() state!: ImportTabState;

    public fileCount: number = 0;
    public files: FileOsMetadata[] = [];
    public importOptions = new AddFileOptions();
    public filters: DialogFilter[] = [
        { name: "Images", extensions: [...IMAGE_EXTENSIONS, ...IMAGE_EXTENSIONS.map(e => e.toUpperCase())] },
        { name: "Videos", extensions: [...VIDEO_EXTENSIONS, ...VIDEO_EXTENSIONS.map(e => e.toUpperCase())] },
        { name: "Audio", extensions: [...AUDIO_EXTENSIONS, ...AUDIO_EXTENSIONS.map(e => e.toUpperCase())] },
        { name: "Documents", extensions: [...DOCUMENT_EXTENSIONS, ...DOCUMENT_EXTENSIONS.map(e => e.toUpperCase())] },
        { name: "Text", extensions: [...TEXT_EXTENSIONS, ...TEXT_EXTENSIONS.map(e => e.toUpperCase())] },
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

    public ngOnInit(): void {
        this.state.selectedPaths.subscribe(paths => {
            this.files = paths;
            this.fileCount = paths.length;
        });
        this.state.importing.subscribe(importing => {
            this.importing = importing;
            this.changeDetector.markForCheck();
        });
        this.state.importedCount.subscribe(count => {
            this.importingProgressTotal = count;
        });
        this.state.importingProgress.subscribe(prog => {
            this.importingProgress = prog;
            this.changeDetector.markForCheck();
        });
    }

    public async setSelectedPaths(paths: string[]) {
        this.changeDetector.markForCheck();
        this.resolving = true;
        try {
            const selectedPaths = await this.importService.resolvePathsToFiles(paths);
            this.state.selectedPaths.next(selectedPaths);
        } catch (err: any) {
            console.log(err);
            this.errorBroker.error(err);
        }
        this.resolving = false;
        this.changeDetector.markForCheck();
    }

    public async import() {
        this.state.importing.next(true);

        this.importingProgress = 0;
        let count = 0;

        for (const file of this.files) {
            try {
                const resultFile = await this.importService.addLocalFile(
                    file,
                    this.importOptions
                );
                this.state.addImportedFile(resultFile);
            } catch (err: any) {
                console.log(err);
                this.errorBroker.error(err);
            }
            count++;
            this.state.importedCount.next(count);
            this.state.importingProgress.next((count / this.fileCount) * 100);
        }

        this.state.importing.next(false);
        this.state.selectedPaths.next([]);
    }
}
