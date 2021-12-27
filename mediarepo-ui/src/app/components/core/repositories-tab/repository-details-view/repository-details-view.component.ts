import {
    Component,
    Input,
    OnChanges, OnDestroy,
    OnInit,
    SimpleChanges
} from "@angular/core";
import {Repository} from "../../../../models/Repository";
import {
    RepositoryService
} from "../../../../services/repository/repository.service";
import {RepositoryMetadata} from "../../../../models/RepositoryMetadata";

@Component({
    selector: "app-repository-details-view",
    templateUrl: "./repository-details-view.component.html",
    styleUrls: ["./repository-details-view.component.scss"]
})
export class RepositoryDetailsViewComponent implements OnInit, OnChanges, OnDestroy {
    @Input() repository!: Repository;

    public metadata?: RepositoryMetadata;
    private refreshMetadataInterval?: number;

    constructor(private repoService: RepositoryService) {
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
        if (this.repository?.local) {
            await this.repoService.closeSelectedRepository();
        } else {
            await this.repoService.disconnectSelectedRepository();
        }
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
            return size + " B"
        }
    }

    public async loadMetadata() {
        this.metadata = await this.repoService.getRepositoryMetadata();
    }
}
