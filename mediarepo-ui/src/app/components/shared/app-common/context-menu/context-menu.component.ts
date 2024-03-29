import {ChangeDetectionStrategy, ChangeDetectorRef, Component, ViewChild,} from "@angular/core";
import {MatMenuTrigger} from "@angular/material/menu";

@Component({
    selector: "app-context-menu",
    templateUrl: "./context-menu.component.html",
    styleUrls: ["./context-menu.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ContextMenuComponent {

    public x: string = "0";
    public y: string = "0";

    @ViewChild(MatMenuTrigger) menuTrigger!: MatMenuTrigger;

    constructor(private changeDetector: ChangeDetectorRef) {
    }

    public onContextMenu(event: MouseEvent) {
        event.preventDefault();
        this.x = event.clientX + "px";
        this.y = event.clientY + "px";
        this.menuTrigger.menu?.focusFirstItem("mouse");
        this.menuTrigger.openMenu();
        this.changeDetector.markForCheck();
    }
}
