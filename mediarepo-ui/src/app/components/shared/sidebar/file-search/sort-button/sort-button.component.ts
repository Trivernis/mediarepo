import {ChangeDetectionStrategy, Component, EventEmitter, Input, Output} from "@angular/core";
import {SortingPreset} from "../../../../../../api/models/SortingPreset";

@Component({
    selector: "app-sort-button",
    templateUrl: "./sort-button.component.html",
    styleUrls: ["./sort-button.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class SortButtonComponent {

    @Input() selectedPreset!: SortingPreset;
    @Output() appClick = new EventEmitter<void>();

    constructor() {
    }
}
