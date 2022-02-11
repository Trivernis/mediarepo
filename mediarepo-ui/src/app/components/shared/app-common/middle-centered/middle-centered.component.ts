import {ChangeDetectionStrategy, Component} from "@angular/core";

@Component({
    selector: "app-middle-centered",
    templateUrl: "./middle-centered.component.html",
    styleUrls: ["./middle-centered.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class MiddleCenteredComponent {

    constructor() {
    }
}
