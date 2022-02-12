import {ChangeDetectionStrategy, Component, Input} from "@angular/core";
import {Tag} from "../../../../../api/models/Tag";

@Component({
    selector: "app-tag-item",
    templateUrl: "./tag-item.component.html",
    styleUrls: ["./tag-item.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class TagItemComponent {

    @Input() tag!: Tag;
    @Input() namespaceColor: string | undefined;
    @Input() tagColor: string | undefined;

    constructor() {
    }
}
