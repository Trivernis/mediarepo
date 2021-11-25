import {Component, OnInit} from "@angular/core";
import {File} from "../../../models/File";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";
import {FileService} from "../../../services/file/file.service";
import {RepositoryService} from "../../../services/repository/repository.service";

@Component({
    selector: "app-files-tab",
    templateUrl: "./files-tab.component.html",
    styleUrls: ["./files-tab.component.scss"]
})
export class FilesTabComponent implements OnInit {


    files: File[] = [];
    contentLoading = false;
    selectedFiles: File[] = [];

    constructor(
        private errorBroker: ErrorBrokerService,
        private repoService: RepositoryService,
        private fileService: FileService,) {
    }

    async ngOnInit() {
        this.fileService.displayedFiles.subscribe(async (files) => {
            this.files = files;
        });
    }

    async onFileSelect(files: File[]) {
        this.selectedFiles = files;
    }
}
