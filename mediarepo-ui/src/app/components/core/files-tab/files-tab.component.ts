import {Component, Input, OnInit} from "@angular/core";
import {File} from "../../../../api/models/File";
import {FilesTabState} from "../../../models/state/FilesTabState";
import {RepositoryMetadata} from "../../../../api/api-types/repo";
import {RepositoryService} from "../../../services/repository/repository.service";
import {TabCategory} from "../../../models/state/TabCategory";
import {take} from "rxjs";

@Component({
    selector: "app-files-tab",
    templateUrl: "./files-tab.component.html",
    styleUrls: ["./files-tab.component.scss"]
})
export class FilesTabComponent implements OnInit {

    @Input() state!: FilesTabState;

    files: File[] = [];
    contentLoading = false;
    selectedFiles: File[] = [];
    public metadata?: RepositoryMetadata;

    private preselectedCd?: string;

    constructor(
        repoService: RepositoryService,
    ) {
        repoService.metadata.subscribe(m => this.metadata = m);
    }

    async ngOnInit() {
        this.state.files.subscribe(files => this.files = files);
        this.state.loading.subscribe(loading => this.contentLoading = loading);
        this.state.files.pipe(take(2)).subscribe(async files => {
            await this.handlePreselection(this.preselectedCd, files);
        });
        this.state.selectedCD.pipe(take(2)).subscribe(async (cd) => {
            await this.handlePreselection(cd, this.files);
        });
    }

    async onFileSelect(files: File[]) {
        this.selectedFiles = files;
        if (files.length === 1) {
            this.state.selectedCD.next(files[0].cd);
        } else {
            this.state.selectedCD.next(undefined);
        }
    }

    public getStateSelectedFile(): File | undefined {
        const hash = this.state.selectedCD.value;

        if (hash) {
            return this.files.find(f => f.cd === hash);
        } else {
            return undefined;
        }
    }

    public async onKeydown(event: KeyboardEvent) {
        switch (event.key) {
            case "F5":
                await this.state.findFiles();
                break;
        }
    }

    public onImportFiles(): void {
        this.state.category = TabCategory.Import;
    }

    private async handlePreselection(cd: string | undefined, files: File[]) {
        console.log(cd, files);
        this.preselectedCd = cd;

        if (cd && files.length > 0) {
            const file = files.find(f => f.cd === cd);

            if (file) {
                console.debug("firing select");
                this.preselectedCd = undefined;
                await this.onFileSelect([file]);
            }
        }
    }
}
