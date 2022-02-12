import {ChangeDetectionStrategy, Component, EventEmitter, Output} from "@angular/core";
import {TabCategory} from "../../../models/state/TabCategory";

type TabCategoryName = "files" | "import";

@Component({
    selector: "app-empty-tab",
    templateUrl: "./empty-tab.component.html",
    styleUrls: ["./empty-tab.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class EmptyTabComponent {

    @Output() tabCategorySelect = new EventEmitter<TabCategory>();

    constructor() {
    }

    public addTab(category: TabCategoryName) {
        switch (category) {
            case "files":
                this.tabCategorySelect.emit(TabCategory.Files);
                break;
            case "import":
                this.tabCategorySelect.emit(TabCategory.Import);
                break;
        }
    }
}
