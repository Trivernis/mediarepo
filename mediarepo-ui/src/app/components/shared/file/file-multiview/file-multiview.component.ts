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

    @ViewChild(FileGalleryComponent) fileGallery!: FileGalleryComponent;
    @ViewChild(FileGridComponent) fileGrid!: FileGridComponent;

    public selectedFiles: File[] = [];
    public preselectedFile: File | undefined;

    constructor() {
    }

    public onFileSelect(files: File[]): void {
        this.selectedFiles = files;
        this.preselectedFile = files[0];
        this.fileSelectEvent.emit(this.selectedFiles);
    }

    public onSinglefileSelect(file: File | undefined): void {
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
        this.mode = "gallery";
        this.fileOpenEvent.emit(file);
    }
}
