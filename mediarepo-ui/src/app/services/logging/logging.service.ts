import {Injectable} from "@angular/core";
import {listen} from "@tauri-apps/api/event";
import {BehaviorSubject} from "rxjs";
import {LogEntry, LogLevel} from "./LogEntry";

@Injectable({
    providedIn: "root"
})
export class LoggingService {

    logs = new BehaviorSubject<LogEntry>(new LogEntry("Log initialized", LogLevel.Trace));

    constructor() {
        this.registerListener().catch(err => console.error(err));
    }

    async registerListener() {
        const _unlisten = await listen("error", event => {
            const payload: any = event.payload;
            if (payload.message) {
                this.error(payload);
            } else {
                this.error(payload.toString());
            }
        });
    }

    async try<T>(fn: () => Promise<T>): Promise<T | undefined> {
        try {
            return await fn();
        } catch (err: any) {
            this.error(err);
            return;
        }
    }

    trace(message: string) {
        this.log(LogLevel.Trace, message);
    }

    debug(message: string) {
        this.log(LogLevel.Debug, message);
    }

    info(message: string) {
        this.log(LogLevel.Info, message);
    }

    warn(message: string) {
        this.log(LogLevel.Warn, message);
    }

    error(error: Error, message?: string) {
        this.log(LogLevel.Error, message ?? error.message ?? error.toString(), error);
    }

    public log(level: LogLevel, message: string, error?: Error) {
        this.logs.next(new LogEntry(message, level, error));
    }
}
