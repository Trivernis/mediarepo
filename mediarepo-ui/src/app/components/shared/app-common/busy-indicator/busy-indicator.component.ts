import {ChangeDetectionStrategy, ChangeDetectorRef, Component, Input, OnChanges, SimpleChanges} from "@angular/core";
import {ProgressSpinnerMode} from "@angular/material/progress-spinner";

@Component({
    selector: "app-busy-indicator",
    templateUrl: "./busy-indicator.component.html",
    styleUrls: ["./busy-indicator.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class BusyIndicatorComponent implements OnChanges {
    @Input() busy: boolean = false;
    @Input() blurBackground: boolean = false;
    @Input() darkenBackground: boolean = false;
    @Input() mode: ProgressSpinnerMode = "indeterminate";
    @Input() indicatorType: "spinner" | "pulse" = "spinner";
    @Input() value: number | undefined;

    constructor(private changeDetector: ChangeDetectorRef) {
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["busy"]) {
            this.changeDetector.markForCheck();
        }
    }

    public setBusy(busy: boolean) {
        if (busy != this.busy) {
            this.busy = busy;
            this.changeDetector.markForCheck();
        }
    }

    public wrapOperation<T>(operation: Function): T | undefined {
        this.setBusy(true);
        try {
            const result = operation();
            this.setBusy(false);
            return result;
        } catch {
            return undefined;
        } finally {
            this.setBusy(false);
        }
    }

    public async wrapAsyncOperation<T>(operation: Function): Promise<T | undefined> {
        this.setBusy(true);
        console.log("busy");
        try {
            const result = await operation();
            this.setBusy(false);
            return result;
        } catch {
            return undefined;
        } finally {
            this.setBusy(false);
            console.log("not busy");
        }
    }
}
