import {
    AfterViewChecked,
    AfterViewInit,
    ChangeDetectionStrategy,
    ChangeDetectorRef,
    Component,
    ElementRef,
    EventEmitter,
    Input,
    OnChanges,
    OnInit,
    Output,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {File} from "../../../../../../api/models/File";
import {FileService} from "../../../../../services/file/file.service";
import {SafeResourceUrl} from "@angular/platform-browser";
import {Selectable} from "../../../../../models/Selectable";
import {TabService} from "../../../../../services/tab/tab.service";
import {Key} from "w3c-keys";

@Component({
    selector: "app-file-gallery",
    templateUrl: "./file-gallery.component.html",
    styleUrls: ["./file-gallery.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FileGalleryComponent implements OnChanges, OnInit, AfterViewInit, AfterViewChecked {

    @Input() files: File[] = [];
    @Input() preselectedFile: File | undefined;
    @Output() fileSelect = new EventEmitter<File | undefined>();
    @Output() fileDblClick = new EventEmitter<File>();
    @Output() appClose = new EventEmitter<FileGalleryComponent>();
    @Output() fileDelete = new EventEmitter<File>();
    @Output() fileDeleted = new EventEmitter<File[]>();

    @ViewChild("inner") inner!: ElementRef<HTMLDivElement>;
    @ViewChild("previewStripContainer") stripContainer!: ElementRef<HTMLDivElement>;

    public entries: Selectable<File>[] = [];
    public selectedFile: Selectable<File> | undefined;
    public fileContentUrl: SafeResourceUrl | undefined;

    public selectedIndex = 0;
    public imageViewHeightPercent = 80;
    public previewStripVisible = true;
    public previewedEntries: (Selectable<File> | undefined)[] = [];

    private previewStripCount = 5;
    private escapeCount = 0;

    constructor(
        private changeDetector: ChangeDetectorRef,
        private tabService: TabService,
        private fileService: FileService
    ) {
    }

    async ngOnInit(): Promise<void> {
        if (!this.selectedFile || this.files.indexOf(
            this.selectedFile.data) < 0) {
            await this.onEntrySelect(
                this.getPreselectedEntry() ?? this.entries[0]);
        } else {
            this.buildPreviewedFiles();
        }
    }

    public ngAfterViewInit(): void {
        this.focus();
    }

    public async ngOnChanges(changes: SimpleChanges): Promise<void> {
        if (changes["files"]) {
            this.entries = this.files.map(
                f => new Selectable(f, f.id == this.selectedFile?.data.id));
            const selectedIndex = this.files.findIndex(
                f => f.id === this.selectedFile?.data.id);

            if (!this.selectedFile || selectedIndex < 0) {
                await this.onEntrySelect(
                    this.getPreselectedEntry() ?? this.entries[0]);
            } else {
                await this.onEntrySelect(this.entries[selectedIndex]);
            }
        }
    }

    public ngAfterViewChecked(): void {
        this.calculatePreviewCount();
    }

    /**
     * Called when a new entry is selected
     * @param {Selectable<File>} entry
     * @returns {Promise<void>}
     */
    async onEntrySelect(entry: Selectable<File>) {
        if (entry) {
            this.selectedFile?.unselect();
            entry.select();
            this.selectedFile = entry;
            this.selectedIndex = this.entries.indexOf(entry);
            await this.loadSelectedFile();

            this.fileSelect.emit(this.selectedFile.data);
            this.buildPreviewedFiles();
        }
    }

    /**
     * Loads the content url of the selected file
     * @returns {Promise<void>}
     */
    async loadSelectedFile() {
        if (this.selectedFile) {
            this.fileContentUrl = this.fileService.buildContentUrl(
                this.selectedFile.data);
        }
    }

    /**
     * Selects the previous item in the gallery
     * @returns {Promise<void>}
     */
    public async nextItem() {
        if (this.selectedFile) {
            let index = this.entries.indexOf(this.selectedFile) + 1;
            if (index == this.entries.length) {
                index--;  // restrict to elements
            }
            await this.onEntrySelect(this.entries[index]);
        } else {
            await this.onEntrySelect(this.entries[0]);
        }
    }

    /**
     * Selects the next item in the gallery
     * @returns {Promise<void>}
     */
    public async previousItem() {
        if (this.selectedFile) {
            let index = this.entries.indexOf(this.selectedFile) - 1;
            if (index < 0) {
                index++; // restrict to elements
            }
            await this.onEntrySelect(this.entries[index]);
        } else {
            await this.onEntrySelect(this.entries[0]);
        }
    }

    public focus() {
        this.inner.nativeElement.focus();
    }

    public async handleKeydownEvent(event: KeyboardEvent) {
        switch (event.key) {
            case Key.ArrowRight:
                await this.nextItem();
                break;
            case Key.ArrowLeft:
                await this.previousItem();
                break;
            case Key.Escape:
                this.onEscapeClick();
                break;
            case Key.Delete:
                if (this.selectedFile) {
                    this.fileDelete.emit(this.selectedFile.data);
                }
                break;
        }
    }

    public trackByFileId(index: number, item?: Selectable<File>) {
        return item?.data.id;
    }

    public togglePreviewStrip(): void {
        if (this.previewStripVisible) {
            this.imageViewHeightPercent = 100;
            this.previewStripVisible = false;
        } else {
            this.imageViewHeightPercent = 80;
            this.previewStripVisible = true;
        }
    }

    public calculatePreviewCount() {
        if (this.stripContainer && this.stripContainer.nativeElement) {
            const width = Math.abs(this.stripContainer.nativeElement.clientWidth);
            const height = Math.abs(this.stripContainer.nativeElement.clientHeight);

            const count = Math.floor(Math.floor(width / height) / 2) * 2 + 1;

            if (count != this.previewStripCount) {
                this.previewStripCount = count;
                this.buildPreviewedFiles();
            }
        }
    }

    public onResize(): void {
        this.changeDetector.markForCheck();
    }

    private getPreselectedEntry(): Selectable<File> | undefined {
        if (this.preselectedFile) {
            const entry = this.entries.find(
                e => e.data.id === this.preselectedFile?.id);
            if (entry) {
                return entry;
            }
        }
        return undefined;
    }

    private onEscapeClick(): void {
        if (this.escapeCount === 1) {
            this.appClose.emit(this);
        } else {
            this.escapeCount++;
            setTimeout(() => this.escapeCount--, 500);
        }
    }

    private buildPreviewedFiles() {
        if (!this.selectedFile) {
            if (this.entries) {
                this.onEntrySelect(this.entries[0]).catch(console.error);
            }
            return;
        }
        const selectedIndex = this.entries.indexOf(this.selectedFile!);
        const previewCountLR = Math.floor(this.previewStripCount / 2);
        const previewedEntries = [];

        for (let i = selectedIndex - previewCountLR; i <= selectedIndex + previewCountLR; i++) {
            if (i >= 0 && i < this.entries.length) {
                previewedEntries.push(this.entries[i]);
            } else {
                previewedEntries.push(undefined);
            }
        }
        this.previewedEntries = previewedEntries;
        this.changeDetector.markForCheck();
    }
}
