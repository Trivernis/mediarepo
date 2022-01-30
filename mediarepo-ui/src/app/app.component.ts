import {Component, OnInit} from "@angular/core";
import {RepositoryService} from "./services/repository/repository.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {LoggingService} from "./services/logging/logging.service";
import {LogEntry, LogLevel} from "./services/logging/LogEntry";
import {environment} from "../environments/environment";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"]
})
export class AppComponent implements OnInit {
    title = "mediarepo-ui";

    constructor(
        private snackBar: MatSnackBar,
        private logger: LoggingService,
        private repoService: RepositoryService,
    ) {
    }

    async ngOnInit() {
        this.logger.logs.subscribe(entry => {
            this.logEntry(entry);
            switch (entry.getLevel()) {
                case LogLevel.Info:
                    this.showInfo(entry.getMessage());
                    break;
                case LogLevel.Warn:
                    this.showWarning(entry.getMessage());
                    break;
                case LogLevel.Error:
                    this.showError(entry.getMessage());
                    break;
            }
        });
        await this.repoService.loadRepositories();
    }

    private showError(err: string) {
        this.snackBar.open(err, undefined, {
            panelClass: "app-error",
            duration: 2000,
        });
    }

    private showWarning(err: string) {
        this.snackBar.open(err, undefined, {
            panelClass: "app-warn",
            duration: 2000,
        });
    }

    private showInfo(info: string) {
        this.snackBar.open(info, undefined, {
            panelClass: "primary",
            duration: 2000,
        });
    }

    private logEntry(entry: LogEntry) {
        if (!environment.production) {
            switch (entry.getLevel()) {
                case LogLevel.Trace:
                    console.trace(entry.getMessage());
                    break;
                case LogLevel.Debug:
                    console.debug(entry.getMessage());
                    break;
                case LogLevel.Info:
                    console.info(entry.getMessage());
                    break;
                case LogLevel.Warn:
                    console.warn(entry.getMessage());
                    break;
            }
        }
        if (entry.getLevel() == LogLevel.Error) {
            console.error(entry.getMessage(), entry.getError());
        }
    }
}
