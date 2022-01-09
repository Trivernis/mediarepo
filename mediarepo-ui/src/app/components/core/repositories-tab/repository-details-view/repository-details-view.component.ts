import {
    Component,
    Input,
    OnChanges,
    OnDestroy,
    OnInit,
    SimpleChanges
} from "@angular/core";
import {Repository} from "../../../../../api/models/Repository";
import {
    RepositoryService
} from "../../../../services/repository/repository.service";
import {RepositoryMetadata} from "../../../../models/RepositoryMetadata";
import {BehaviorSubject} from "rxjs";
import {MatDialog} from "@angular/material/dialog";
import {
    BusyDialogComponent
} from "../../../shared/app-common/busy-dialog/busy-dialog.component";

@Component({
    selector: "app-repository-details-view",
    templateUrl: "./repository-details-view.component.html",
    styleUrls: ["./repository-details-view.component.scss"]
})
export class RepositoryDetailsViewComponent implements OnInit, OnChanges, OnDestroy {
    @Input() repository!: Repository;

    public metadata?: RepositoryMetadata;
    private refreshMetadataInterval?: number;

    public totalSize = new BehaviorSubject<string | undefined>(undefined);
    public fileFolderSize = new BehaviorSubject<string | undefined>(undefined);
    public thumbFolderSize = new BehaviorSubject<string | undefined>(undefined);
    public databaseFileSize = new BehaviorSubject<string | undefined>(undefined);

    constructor(private repoService: RepositoryService, public dialog: MatDialog) {
    }

    public async ngOnInit() {
        await this.loadMetadata();
        this.refreshMetadataInterval = setInterval(async () => this.loadMetadata(), 30000);
    }

    public async ngOnChanges(changes: SimpleChanges) {
        if (changes["repository"]) {
            await this.loadMetadata();
        }
    }

    public ngOnDestroy(): void {
        clearInterval(this.refreshMetadataInterval);
    }

    public async closeRepository() {
        let closeDialog = this.dialog.open(BusyDialogComponent, {
            data: {
                title: "Closing repository",
                message: new BehaviorSubject("Closing repository...")
            }
        });
        if (this.repository?.local) {
            await this.repoService.closeSelectedRepository();
        } else {
            await this.repoService.disconnectSelectedRepository();
        }
        closeDialog.close(true);
    }

    public async getSizes() {
        const totalSize = await this.repoService.getSize("Total");
        this.totalSize.next(this.formatByteSize(totalSize.size));
        const fileSize = await this.repoService.getSize("FileFolder");
        this.fileFolderSize.next(this.formatByteSize(fileSize.size));
        const thumbSize = await this.repoService.getSize("ThumbFolder");
        this.thumbFolderSize.next(this.formatByteSize(thumbSize.size));
        const databaseSize = await this.repoService.getSize("DatabaseFile");
        this.databaseFileSize.next(this.formatByteSize(databaseSize.size));
    }

    public formatByteSize(size: number): string {
        const kib = 1024;
        const mib = kib ** 2;
        const gib = kib ** 3;
        const tib = kib ** 4;

        if (size >= tib) {
            return (size / tib).toFixed(2) + " TiB";
        } else if (size >= gib) {
            return (size / gib).toFixed(2) + " GiB";
        } else if (size >= mib) {
            return (size / mib).toFixed(2) + " MiB";
        } else if (size >= kib) {
            return (size / kib).toFixed(2) + " KiB";
        } else {
            return size + " B";
        }
    }

    public async loadMetadata() {
        this.metadata = await this.repoService.getRepositoryMetadata();
        await this.getSizes();
    }
}
