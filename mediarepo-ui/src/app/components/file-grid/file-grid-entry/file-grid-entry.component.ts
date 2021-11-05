import {
    Component,
    ElementRef,
    EventEmitter,
    Input,
    OnChanges,
    OnInit,
    Output,
    ViewChild
} from '@angular/core';
import {File} from "../../../models/File";
import {FileService} from "../../../services/file/file.service";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";
import {SafeResourceUrl} from "@angular/platform-browser";
import {GridEntry} from "./GridEntry";

@Component({
    selector: 'app-file-grid-entry',
    templateUrl: './file-grid-entry.component.html',
    styleUrls: ['./file-grid-entry.component.scss']
})
export class FileGridEntryComponent implements OnInit, OnChanges {

    @ViewChild("card") card!: ElementRef;
    @Input() public gridEntry!: GridEntry;
    @Output() clickEvent = new EventEmitter<FileGridEntryComponent>();
    @Output() dblClickEvent = new EventEmitter<FileGridEntryComponent>();

    contentUrl: SafeResourceUrl | undefined;
    private cachedFile: File | undefined;

    constructor(private fileService: FileService, private errorBroker: ErrorBrokerService) {
    }

    async ngOnInit() {
        this.cachedFile = this.gridEntry.file;
        await this.loadImage();
    }

    async ngOnChanges() {
        if (!this.cachedFile || this.gridEntry.file.hash !== this.cachedFile.hash) {
            this.cachedFile = this.gridEntry.file;
            await this.loadImage();
        }
    }

    async loadImage() {
        try {
            const thumbnails = await this.fileService.getThumbnails(
                this.gridEntry.file.hash);
            let thumbnail = thumbnails.find(
                t => (t.height > 250 || t.width > 250) && (t.height < 500 && t.width < 500));
            thumbnail = thumbnail ?? thumbnails[0];

            if (!thumbnail) {
                console.log("Thumbnail is empty?!", thumbnails);
            } else {
                this.contentUrl = await this.fileService.readThumbnail(
                    thumbnail!!);
            }
        } catch (err) {
            this.errorBroker.showError(err);
        }
    }
}
