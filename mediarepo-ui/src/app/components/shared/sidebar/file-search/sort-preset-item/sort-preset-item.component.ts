import {ChangeDetectionStrategy, Component, Input} from "@angular/core";
import {SortingPreset} from "../../../../../../api/models/SortingPreset";

@Component({
    selector: "app-sort-preset-item",
    templateUrl: "./sort-preset-item.component.html",
    styleUrls: ["./sort-preset-item.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class SortPresetItemComponent {

    @Input() preset!: SortingPreset;

    constructor() {
    }
}
