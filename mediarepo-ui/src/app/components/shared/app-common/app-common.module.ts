import {NgModule} from "@angular/core";
import {
    ConfirmDialogComponent
} from "./confirm-dialog/confirm-dialog.component";
import {
    BusyIndicatorComponent
} from "./busy-indicator/busy-indicator.component";
import {ContextMenuComponent} from "./context-menu/context-menu.component";
import {CommonModule} from "@angular/common";
import {NgIconsModule} from "@ng-icons/core";
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {MatButtonModule} from "@angular/material/button";
import {MatDialogModule} from "@angular/material/dialog";
import {MatMenuModule} from "@angular/material/menu";
import {
    ContentAwareImageComponent
} from "./content-aware-image/content-aware-image.component";
import { InputReceiverDirective } from "./input-receiver/input-receiver.directive";


@NgModule({
    declarations: [
        ConfirmDialogComponent,
        BusyIndicatorComponent,
        ContextMenuComponent,
        ContentAwareImageComponent,
        InputReceiverDirective,
    ],
    exports: [
        ConfirmDialogComponent,
        BusyIndicatorComponent,
        ContextMenuComponent,
        ContentAwareImageComponent,
        InputReceiverDirective,
    ],
    imports: [
        CommonModule,
        NgIconsModule.withIcons({}),
        MatProgressSpinnerModule,
        MatButtonModule,
        MatDialogModule,
        MatMenuModule
    ]
})
export class AppCommonModule {
}
