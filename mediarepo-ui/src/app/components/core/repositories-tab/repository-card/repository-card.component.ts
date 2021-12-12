import {Component, Input, OnDestroy, OnInit, ViewChild} from "@angular/core";
import {Repository} from "../../../../models/Repository";
import {
    RepositoryService
} from "../../../../services/repository/repository.service";
import {
    ErrorBrokerService
} from "../../../../services/error-broker/error-broker.service";
import {MatDialog} from "@angular/material/dialog";
import {
    ConfirmDialogComponent
} from "../../../shared/app-common/confirm-dialog/confirm-dialog.component";
import {
    BusyIndicatorComponent
} from "../../../shared/app-common/busy-indicator/busy-indicator.component";
import {
    EditRepositoryDialogComponent
} from "../edit-repository-dialog/edit-repository-dialog.component";

@Component({
    selector: "app-repository-card",
    templateUrl: "./repository-card.component.html",
    styleUrls: ["./repository-card.component.scss"]
})
export class RepositoryCardComponent implements OnInit, OnDestroy {

    @Input() repository!: Repository;
    @ViewChild(BusyIndicatorComponent) busyIndicator!: BusyIndicatorComponent;

    public daemonRunning: boolean = false;

    statusRefreshInterval: number | undefined;

    constructor(
        public repoService: RepositoryService,
        private errorBroker: ErrorBrokerService,
        public dialog: MatDialog) {
    }

    public async ngOnInit() {
        if (!this.repository.local) {
            await this.checkRemoteRepositoryStatus();
            this.statusRefreshInterval = setInterval(
                async () => await this.checkRemoteRepositoryStatus(), 10000);
        }
    }

    public async ngOnDestroy(): Promise<void> {
        if (this.statusRefreshInterval != undefined) {
            clearInterval(this.statusRefreshInterval);
        }
    }

    public isSelectedRepository(): boolean {
        return this.repoService.selectedRepository.getValue()?.name === this.repository.name
    }

    public async removeRepository() {
        const confirmation = await this.dialog.open(ConfirmDialogComponent, {
            data: {
                title: "Remove repository",
                message: `Do you really want to remove the repository "${this.repository.name}"?`,
                confirmAction: "Remove",
                confirmColor: "warn"
            }
        }).afterClosed().toPromise();
        if (confirmation === true) {
            if (this.isSelectedRepository()) {
                if (this.repository.local) {
                    await this.repoService.closeSelectedRepository();
                } else {
                    await this.repoService.disconnectSelectedRepository();
                }
            }
            await this.promtDeleteRepository();
        }
    }

    private async promtDeleteRepository() {
        if (this.repository.local) {
            const deleteContents = await this.dialog.open(
                ConfirmDialogComponent, {
                    data: {
                        title: "Delete repository content",
                        message: "Do you want to remove the contents of the repository as well?",
                        confirmAction: "Delete",
                        confirmColor: "warn",
                        denyAction: "No",
                    }
                }).afterClosed().toPromise();
            if (deleteContents) {
                await this.repoService.deleteRepository(this.repository.name);
            } else {
                await this.repoService.removeRepository(this.repository.name);
            }
        } else {
            await this.repoService.removeRepository(this.repository.name);
        }
    }

    public getDaemonStatusText(): string {
        if (this.repository.local) {
            return "Local";
        } else if (this.daemonRunning) {
            return "Online";
        } else {
            return "Offline";
        }
    }

    public getDaemonStatusClass(): string {
        if (this.repository.local) {
            return "status-local";
        } else if (this.daemonRunning) {
            return "status-online";
        } else {
            return "status-offline";
        }
    }

    public async startDaemonAndSelectRepository() {
        try {
            if (!this.daemonRunning) {
                await this.repoService.startDaemon(this.repository.path!);
                this.daemonRunning = true;
                await new Promise((res, _) => {
                    setTimeout(res, 2000) // wait for the daemon to start
                });
            }
            await this.selectRepository();
        } catch (err) {
            this.errorBroker.showError(err);
        }
    }

    public async selectRepository() {
        this.busyIndicator.setBusy(true);
        try {
            await this.repoService.setRepository(this.repository);
        } catch (err) {
            this.errorBroker.showError(err);
        }
        this.busyIndicator.setBusy(false);
    }

    async checkRemoteRepositoryStatus() {
        this.daemonRunning = await this.repoService.checkDaemonRunning(
            this.repository.address!);
    }

    public openEditRepositoryDialog(): void {
        this.dialog.open(EditRepositoryDialogComponent, {
            disableClose: true,
            minWidth: "30%",
            minHeight: "30%",
            data: {
                repository: this.repository
            }
        })
    }
}
