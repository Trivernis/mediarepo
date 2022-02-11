import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from "@angular/core";

export type Attachment = "top" | "bottom" | "left" | "right";
export type Alignment = "start" | "center" | "end";

@Component({
    selector: "app-flap-button",
    templateUrl: "./flap-button.component.html",
    styleUrls: ["./flap-button.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FlapButtonComponent {

    @Input() attach: Attachment = "top";
    @Input() align: Alignment = "center";
    @Output() appClick = new EventEmitter<MouseEvent>();

    constructor() {
    }
}
