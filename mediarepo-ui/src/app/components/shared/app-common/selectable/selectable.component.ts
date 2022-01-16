import {Component, EventEmitter, Output} from "@angular/core";

@Component({
    selector: "app-selectable",
    templateUrl: "./selectable.component.html",
    styleUrls: ["./selectable.component.scss"]
})
export class SelectableComponent {
    public selected = false;

    @Output() appSelect = new EventEmitter<this>();
    @Output() appUnselect = new EventEmitter<this>();

    constructor() {
    }

    public onClick(): void {
        this.selected = !this.selected;
        if (this.selected) {
            this.appSelect.emit(this);
        } else {
            this.appUnselect.emit(this);
        }
    }
}
