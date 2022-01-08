import {AfterViewInit, Component, OnInit} from "@angular/core";
import {Repository} from "../../../../api/models/Repository";
import {
    RepositoryService
} from "../../../services/repository/repository.service";
import {MatDialog} from "@angular/material/dialog";
import {
    DownloadDaemonDialogComponent
} from "./download-daemon-dialog/download-daemon-dialog.component";
import {
    AddRepositoryDialogComponent
} from "../../shared/repository/repository/add-repository-dialog/add-repository-dialog.component";

@Component({
    selector: "app-repositories-tab",
    templateUrl: "./repositories-tab.component.html",
    styleUrls: ["./repositories-tab.component.scss"]
})
export class RepositoriesTabComponent implements OnInit, AfterViewInit {
    public repositories: Repository[] = [];
    public selectedRepository?: Repository;

    constructor(
        private repoService: RepositoryService,
        public dialog: MatDialog
    ) {
    }

    ngOnInit(): void {
        this.repoService.repositories.subscribe({
            next: (repos) => {
                this.repositories = repos;
            }
        });
        this.repoService.selectedRepository.subscribe(repo => this.selectedRepository = repo);
    }

    public async ngAfterViewInit() {
        await this.checkAndPromptDaemonExecutable();
    }

    public openAddRepositoryDialog() {
        this.dialog.open(AddRepositoryDialogComponent, {
            disableClose: true,
            minWidth: "30%",
            minHeight: "30%",
        });
    }

    private async checkAndPromptDaemonExecutable() {
        if (!await this.repoService.checkDameonConfigured()) {
            const result = await this.dialog.open(DownloadDaemonDialogComponent, {
                disableClose: true,
            }).afterClosed().toPromise();
            if (result) {
                // recursion avoidance
                setTimeout(async () => await this.checkAndPromptDaemonExecutable(), 0);
            }
        }
    }
}
