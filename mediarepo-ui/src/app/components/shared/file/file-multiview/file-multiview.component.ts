import {
    Component,
    ElementRef,
    EventEmitter,
    Input,
    Output,
    ViewChild
} from "@angular/core";
import {File} from "../../../../models/File";
import {FileGalleryComponent} from "./file-gallery/file-gallery.component";
import {FileGridComponent} from "./file-grid/file-grid.component";

@Component({
    selector: "app-file-multiview",
    templateUrl: "./file-multiview.component.html",
    styleUrls: ["./file-multiview.component.scss"]
})
export class FileMultiviewComponent {

    @Input() files!: File[];
    @Input() mode: "grid" | "gallery" = "grid";

    @Output() fileOpenEvent = new EventEmitter<File>();
    @Output() fileSelectEvent = new EventEmitter<File[]>();
    @Output() modeChangeEvent = new EventEmitter<"grid"|"gallery">();

    @ViewChild(FileGalleryComponent) fileGallery!: FileGalleryComponent;
    @ViewChild(FileGridComponent) fileGrid!: FileGridComponent;

    public selectedFiles: File[] = [];
    @Input() public preselectedFile: File | undefined;

    constructor() {
    }

    public onFileSelect(files: File[]): void {
        this.selectedFiles = files;
        this.preselectedFile = files[0];
        this.fileSelectEvent.emit(this.selectedFiles);
    }

    public onSingleFileSelect(file: File | undefined): void {
        if (file) {
            this.selectedFiles = [file];
            this.preselectedFile = file;
        } else {
            this.selectedFiles = [];
        }
        this.fileSelectEvent.emit(this.selectedFiles);
    }

    public onFileOpen(file: File): void {
        this.preselectedFile = file;
        this.setMode("gallery")
        this.fileOpenEvent.emit(file);
    }

    public setMode(mode: "grid" | "gallery") {
        this.mode = mode;
        this.modeChangeEvent.emit(mode);
    }
}
