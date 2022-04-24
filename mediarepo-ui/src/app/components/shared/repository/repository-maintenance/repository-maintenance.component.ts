import {ChangeDetectionStrategy, ChangeDetectorRef, Component, OnDestroy, OnInit} from "@angular/core";
import {JobService} from "../../../../services/job/job.service";
import {JobType} from "../../../../../api/api-types/job";
import {MatDialog} from "@angular/material/dialog";
import {BusyDialogComponent, BusyDialogData} from "../../app-common/busy-dialog/busy-dialog.component";
import {LoggingService} from "../../../../services/logging/logging.service";
import {BehaviorSubject} from "rxjs";


@Component({
    selector: "app-repository-maintenance",
    templateUrl: "./repository-maintenance.component.html",
    styleUrls: ["./repository-maintenance.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class RepositoryMaintenanceComponent implements OnInit, OnDestroy {
    public jobState: { [Property in JobType]?: boolean } = {
        CalculateSizes: false,
        GenerateThumbnails: false,
    };
    private jobStatusInterval: any;

    constructor(
        private changeDetector: ChangeDetectorRef,
        private jobService: JobService,
        private dialog: MatDialog,
        private logger: LoggingService
    ) {
    }

    public ngOnDestroy(): void {
        clearInterval(this.jobStatusInterval);
    }

    public async ngOnInit() {
        await this.updateJobStatus();
        this.jobStatusInterval = setInterval(() => this.updateJobStatus(), 10000);
    }

    public async runJob(jobType: JobType, runAsync: boolean) {
        if (runAsync) {
            this.jobState[jobType] = true;
            this.jobService.runJob(jobType).then(() => this.delay(1000)).catch(this.logger.error).finally(() => {
                this.jobState[jobType] = false;
                this.changeDetector.markForCheck();
            });
            this.changeDetector.markForCheck();
        } else {
            const dialog = this.dialog.open<BusyDialogComponent, BusyDialogData>(BusyDialogComponent, {
                disableClose: true,
                minWidth: "30%",
                minHeight: "30%",
                data: {
                    title: "Synchronous Job",
                    message: new BehaviorSubject(`Running Job ${jobType}`),
                    allowCancel: false,
                }
            });
            try {
                this.changeDetector.markForCheck();
                await this.jobService.runJob(jobType);
            } catch (err: any) {
                this.logger.error(err);
            } finally {
                dialog.close();
                this.changeDetector.markForCheck();
            }
        }
    }

    private async delay(ms: number) {
        return new Promise((res, _) => setTimeout(
            res,
            ms
        ));
    }

    private async updateJobStatus() {
        const indexedTypes: JobType[] = ["CalculateSizes", "GenerateThumbnails"];
        for (const jobType of indexedTypes) {
            this.jobState[jobType] = await this.jobService.isJobRunning(jobType);
        }
        this.changeDetector.markForCheck();
    }
}
