import {ChangeDetectionStrategy, Component, Input} from "@angular/core";
import {shell} from "@tauri-apps/api";

@Component({
    selector: "app-external-url",
    templateUrl: "./external-url.component.html",
    styleUrls: ["./external-url.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ExternalUrlComponent {

    @Input() href!: string;
    private opening = false;

    constructor() {
    }

    public async openUrl() {
        if (this.opening) {
            return;
        }
        this.opening = true;
        try {
            await shell.open(this.href);
        } catch (err) {
            console.error(err);
        } finally {
            this.opening = false;
        }
    }
}
