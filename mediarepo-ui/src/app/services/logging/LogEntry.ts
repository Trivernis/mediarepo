export enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

export class LogEntry {
    constructor(private message: string, private level: LogLevel, private error?: Error) {
    }

    public getMessage(): string {
        return this.message;
    }

    public getLevel(): LogLevel {
        return this.level;
    }

    public getError(): Error | undefined {
        return this.error;
    }
}
