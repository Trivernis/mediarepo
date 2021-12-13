import {
    AfterContentInit, AfterViewInit,
    Component, ElementRef,
    EventEmitter,
    HostListener,
    Input,
    OnChanges,
    OnInit,
    Output,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {File} from "../../../../../models/File";
import {FileService} from "../../../../../services/file/file.service";
import {SafeResourceUrl} from "@angular/platform-browser";
import {Selectable} from "../../../../../models/Selectable";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";
import {TabService} from "../../../../../services/tab/tab.service";

@Component({
    selector: "app-file-gallery",
    templateUrl: "./file-gallery.component.html",
    styleUrls: ["./file-gallery.component.scss"]
})
export class FileGalleryComponent implements OnChanges, OnInit, AfterViewInit {

    @Input() files: File[] = [];
    @Input() preselectedFile: File | undefined;
    @Output() fileSelectEvent = new EventEmitter<File | undefined>();
    @Output() fileDblClickEvent = new EventEmitter<File>();
    @Output() closeEvent = new EventEmitter<FileGalleryComponent>();
    entries: Selectable<File>[] = [];

    @ViewChild("virtualScroll") virtualScroll!: CdkVirtualScrollViewport;
    @ViewChild("inner") inner!: ElementRef<HTMLDivElement>;

    public selectedFile: Selectable<File> | undefined;
    public fileContentUrl: SafeResourceUrl | undefined;

    private scrollTimeout: number | undefined;
    private escapeCount = 0;

    constructor(private tabService: TabService, private fileService: FileService) {
        tabService.selectedTab.subscribe(() => this.adjustElementSizes());
    }

    async ngOnInit(): Promise<void> {
        if (!this.selectedFile || this.files.indexOf(
            this.selectedFile.data) < 0) {
            await this.onEntrySelect(
                this.getPreselectedEntry() ?? this.entries[0])
        }
    }

    public ngAfterViewInit(): void {
        this.focus();
    }

    public async ngOnChanges(changes: SimpleChanges): Promise<void> {
        if (changes["files"]) {
            this.entries = this.files.map(
                f => new Selectable(f, f.hash == this.selectedFile?.data.hash));
            const selectedIndex = this.files.findIndex(
                f => f.hash === this.selectedFile?.data.hash);

            if (!this.selectedFile || selectedIndex < 0) {
                await this.onEntrySelect(
                    this.getPreselectedEntry() ?? this.entries[0])
            } else {
                await this.onEntrySelect(this.entries[selectedIndex])
            }
        }
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
            await this.loadSelectedFile();

            if (this.virtualScroll) {
                clearTimeout(this.scrollTimeout);
                this.scrollTimeout = setTimeout(() => this.scrollToSelection(),
                    0);  // we need to make sure the viewport has rendered
            }

            this.fileSelectEvent.emit(this.selectedFile.data);
        }
    }

    /**
     * Loads the content url of the selected file
     * @returns {Promise<void>}
     */
    async loadSelectedFile() {
        if (this.selectedFile) {
            this.fileContentUrl = this.fileService.buildContentUrl(
                this.selectedFile.data)
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
            await this.onEntrySelect(this.entries[0])
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
            await this.onEntrySelect(this.entries[0])
        }
    }

    public adjustElementSizes(): void {
        if (this.virtualScroll) {
            this.virtualScroll.checkViewportSize();
            this.scrollToSelection();
        }
    }

    public focus() {
        this.inner.nativeElement.focus();
    }

    public async handleKeydownEvent(event: KeyboardEvent) {
        switch (event.key) {
            case "ArrowRight":
                await this.nextItem();
                break;
            case "ArrowLeft":
                await this.previousItem();
                break;
            case "Escape":
                this.onEscapeClick();
                break;
        }
    }

    private scrollToSelection(): void {
        if (this.selectedFile) {
            const selectedIndex = this.entries.indexOf(this.selectedFile);
            const viewportSize = this.virtualScroll.getViewportSize();
            const indexAdjustment = (viewportSize / 260) / 2; // adjustment to have the selected item centered
            this.virtualScroll.scrollToIndex(
                Math.max(selectedIndex - indexAdjustment, 0), "smooth");

            if (selectedIndex > indexAdjustment) {
                this.virtualScroll.scrollToOffset(
                    this.virtualScroll.measureScrollOffset("left") + 130,
                    "smooth");
            }
        }
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
            this.closeEvent.emit(this);
        } else {
            this.escapeCount++;
            setTimeout(() => this.escapeCount--, 500);
        }
    }
}
