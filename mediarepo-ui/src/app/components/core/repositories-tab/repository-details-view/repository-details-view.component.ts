import {
    Component,
    Input,
    OnChanges,
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
export class RepositoryDetailsViewComponent implements OnInit, OnChanges {
    @Input() repository!: Repository;

    public metadata?: RepositoryMetadata;

    constructor(private repoService: RepositoryService) {
    }

    public async ngOnInit() {
        this.metadata = await this.repoService.getRepositoryMetadata();
    }

    public async ngOnChanges(changes: SimpleChanges) {
        if (changes["repository"]) {
            this.metadata = await this.repoService.getRepositoryMetadata();
        }
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
}
