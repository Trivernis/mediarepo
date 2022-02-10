import {ChangeDetectionStrategy, ChangeDetectorRef, Component, EventEmitter, Output} from "@angular/core";

@Component({
    selector: "app-drawer-page",
    templateUrl: "./drawer-page.component.html",
    styleUrls: ["./drawer-page.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class DrawerPageComponent {

    public drawerOpened = true;

    @Output() appSizeChange = new EventEmitter<void>();

    constructor(private changeDetecter: ChangeDetectorRef) {
    }

    public toggleDrawer(): void {
        this.drawerOpened = !this.drawerOpened;
        this.appSizeChange.emit();
        this.changeDetecter.markForCheck();
    }
}
