import {AfterViewInit, Component, OnInit} from "@angular/core";
import {Repository} from "../../../../api/models/Repository";
import {RepositoryService} from "../../../services/repository/repository.service";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {DownloadDaemonDialogComponent} from "./download-daemon-dialog/download-daemon-dialog.component";
import {
    AddRepositoryDialogComponent
} from "../../shared/repository/repository/add-repository-dialog/add-repository-dialog.component";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";
import {BehaviorSubject} from "rxjs";
import {BusyDialogComponent} from "../../shared/app-common/busy-dialog/busy-dialog.component";
import {JobService} from "../../../services/job/job.service";
import {StateService} from "../../../services/state/state.service";

type BusyDialogContext = { message: BehaviorSubject<string>, dialog: MatDialogRef<BusyDialogComponent> };

@Component({
    selector: "app-repositories-tab",
    templateUrl: "./repositories-tab.component.html",
    styleUrls: ["./repositories-tab.component.scss"]
})
export class RepositoriesTabComponent implements OnInit, AfterViewInit {
    public repositories: Repository[] = [];
    public selectedRepository?: Repository;

    constructor(
        private errorBroker: ErrorBrokerService,
        private repoService: RepositoryService,
        private jobService: JobService,
        private stateService: StateService,
        public dialog: MatDialog
    ) {
    }

    ngOnInit(): void {
        this.repoService.repositories.subscribe({
            next: (repos) => {
                this.repositories = repos;
            }
        });
        this.repoService.selectedRepository.subscribe(
            repo => this.selectedRepository = repo);
    }

    public async ngAfterViewInit() {
        await this.checkAndPromptDaemonExecutable();
    }

    public async startDaemonAndSelectRepository(repository: Repository) {
        try {
            let dialogContext = this.openStartupDialog(repository);
            let daemonRunning = await this.repoService.checkDaemonRunning(
                repository.path!);
            if (!daemonRunning) {
                dialogContext.message.next("Starting repository daemon...");
                await this.repoService.startDaemon(repository.path!);

                await new Promise((res, _) => {
                    setTimeout(res, 2000); // wait for the daemon to start
                });
            }
            await this.selectRepository(repository, dialogContext);
        } catch (err) {
            this.errorBroker.showError(err);
        }
    }

    public async selectRepository(repository: Repository, dialogContext?: BusyDialogContext) {
        dialogContext = dialogContext ?? this.openStartupDialog(repository);
        try {
            dialogContext.message.next("Opening repository...");
            await this.repoService.setRepository(repository);
            await this.runRepositoryStartupTasks(dialogContext);
            dialogContext.message.next("Restoring previous tabs...");
            await this.repoService.loadRepositories();
            dialogContext.dialog.close(true);
        } catch (err: any) {
            this.errorBroker.showError(err);
            dialogContext.message.next(
                "Failed to open repository: " + err.toString());
            await this.forceCloseRepository();
            setTimeout(() => dialogContext!.dialog.close(true), 1000);
        }
    }

    public openAddRepositoryDialog() {
        this.dialog.open(AddRepositoryDialogComponent, {
            disableClose: true,
            minWidth: "30%",
            minHeight: "30%",
        });
    }

    public async onOpenRepository(repository: Repository) {
        if (!repository.local) {
            await this.selectRepository(repository);
        } else {
            await this.startDaemonAndSelectRepository(repository);
        }
    }

    private async forceCloseRepository() {
        try {
            await this.repoService.closeSelectedRepository();
        } catch {
        }
        try {
            await this.repoService.disconnectSelectedRepository();
        } catch {
        }
    }

    private async runRepositoryStartupTasks(dialogContext: BusyDialogContext): Promise<void> {
        dialogContext.message.next(
            "Migrating content descriptors to new format...");
        await this.jobService.runJob("MigrateContentDescriptors");
        dialogContext.message.next("Calculating repository sizes...");
        await this.jobService.runJob("CalculateSizes");
        dialogContext.message.next("Checking integrity...");
        await this.jobService.runJob("CheckIntegrity");
        dialogContext.message.next("Finished repository startup");
    }

    private openStartupDialog(repository: Repository): BusyDialogContext {
        const dialogMessage = new BehaviorSubject<string>(
            "Opening repository...");
        let dialog = this.dialog.open(BusyDialogComponent, {
            data: {
                title: `Opening repository ${repository.name}`,
                message: dialogMessage,
                allowCancel: true,
            }, disableClose: true,
            minWidth: "30%",
            minHeight: "30%",
        });
        dialog.afterClosed().subscribe(async (result) => {
            if (!result) {
                await this.forceCloseRepository();
            }
        });

        return { message: dialogMessage, dialog };
    }

    private async checkAndPromptDaemonExecutable() {
        if (!await this.repoService.checkDameonConfigured()) {
            const result = await this.dialog.open(
                DownloadDaemonDialogComponent,
                {
                    disableClose: true,
                }
            ).afterClosed().toPromise();
            if (result) {
                // recursion avoidance
                setTimeout(
                    async () => await this.checkAndPromptDaemonExecutable(), 0);
            }
        }
    }
}
