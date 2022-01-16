import {AfterViewInit, Component, EventEmitter, Input, Output, ViewChild} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {FileGalleryComponent} from "./file-gallery/file-gallery.component";
import {FileGridComponent} from "./file-grid/file-grid.component";
import {FileActionBaseComponent} from "../../app-base/file-action-base/file-action-base.component";
import {MatDialog} from "@angular/material/dialog";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {FileService} from "../../../../services/file/file.service";

@Component({
    selector: "app-file-multiview",
    templateUrl: "./file-multiview.component.html",
    styleUrls: ["./file-multiview.component.scss"]
})
export class FileMultiviewComponent extends FileActionBaseComponent implements AfterViewInit {

    @Input() files!: File[];
    @Input() mode: "grid" | "gallery" = "grid";

    @Output() fileOpenEvent = new EventEmitter<File>();
    @Output() fileSelectEvent = new EventEmitter<File[]>();
    @Output() modeChangeEvent = new EventEmitter<"grid" | "gallery">();

    @ViewChild(FileGalleryComponent) fileGallery!: FileGalleryComponent;
    @ViewChild(FileGridComponent) fileGrid!: FileGridComponent;

    public selectedFiles: File[] = [];
    @Input() public preselectedFile: File | undefined;

    constructor(dialog: MatDialog, errorBroker: ErrorBrokerService, fileService: FileService) {
        super(dialog, errorBroker, fileService);
    }

    public ngAfterViewInit(): void {
        if (this.preselectedFile) {
            this.fileSelectEvent.emit([this.preselectedFile]);
            this.selectedFiles = [this.preselectedFile];
        }
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
        this.setMode("gallery");
        this.fileOpenEvent.emit(file);
    }

    public setMode(mode: "grid" | "gallery") {
        this.mode = mode;
        this.modeChangeEvent.emit(mode);
    }

    public async onFileDelete(files: File[]): Promise<void> {
        let deletePermanently = true;

        for (const file of files) {
            deletePermanently &&= file.status === "Deleted";
        }

        if (deletePermanently) {
            await this.deletePermanently(files);
        } else {
            await this.updateStatus(files, "Deleted");
        }
    }
}
