import {ChangeDetectionStrategy, Component, OnInit} from "@angular/core";

@Component({
    selector: "app-drawer-page-side",
    templateUrl: "./drawer-page-side.component.html",
    styleUrls: ["./drawer-page-side.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class DrawerPageSideComponent implements OnInit {

    constructor() {
    }

    ngOnInit(): void {
    }

}
