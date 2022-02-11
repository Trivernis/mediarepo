import {NgModule} from "@angular/core";
import {ConfirmDialogComponent} from "./confirm-dialog/confirm-dialog.component";
import {BusyIndicatorComponent} from "./busy-indicator/busy-indicator.component";
import {ContextMenuComponent} from "./context-menu/context-menu.component";
import {CommonModule} from "@angular/common";
import {NgIconsModule} from "@ng-icons/core";
import {MatChevronLeft, MatChevronRight} from "@ng-icons/material-icons";
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {MatButtonModule} from "@angular/material/button";
import {MatDialogModule} from "@angular/material/dialog";
import {MatMenuModule} from "@angular/material/menu";
import {ContentAwareImageComponent} from "./content-aware-image/content-aware-image.component";
import {InputReceiverDirective} from "./input-receiver/input-receiver.directive";
import {MetadataEntryComponent} from "./metadata-entry/metadata-entry.component";
import {BusyDialogComponent} from "./busy-dialog/busy-dialog.component";
import {SelectableComponent} from "./selectable/selectable.component";
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {HasPropertyPipe} from "./pipes/has-property.pipe";
import {DrawerPageComponent} from "./drawer-page/drawer-page.component";
import {MatSidenavModule} from "@angular/material/sidenav";
import {DrawerPageSideComponent} from "./drawer-page/drawer-page-side/drawer-page-side.component";
import {DrawerPageContentComponent} from "./drawer-page/drawer-page-content/drawer-page-content.component";
import {FlexLayoutModule} from "@angular/flex-layout";
import {MatRippleModule} from "@angular/material/core";
import {FlapButtonComponent} from "./flap-button/flap-button.component";
import {MiddleCenteredComponent} from "./middle-centered/middle-centered.component";


@NgModule({
    declarations: [
        ConfirmDialogComponent,
        BusyIndicatorComponent,
        ContextMenuComponent,
        ContentAwareImageComponent,
        InputReceiverDirective,
        MetadataEntryComponent,
        BusyDialogComponent,
        SelectableComponent,
        HasPropertyPipe,
        DrawerPageComponent,
        DrawerPageSideComponent,
        DrawerPageContentComponent,
        FlapButtonComponent,
        MiddleCenteredComponent,
    ],
    exports: [
        ConfirmDialogComponent,
        BusyIndicatorComponent,
        ContextMenuComponent,
        ContentAwareImageComponent,
        InputReceiverDirective,
        MetadataEntryComponent,
        SelectableComponent,
        HasPropertyPipe,
        DrawerPageComponent,
        DrawerPageSideComponent,
        DrawerPageContentComponent,
        FlapButtonComponent,
        MiddleCenteredComponent,
    ],
    imports: [
        CommonModule,
        NgIconsModule.withIcons({ MatChevronLeft, MatChevronRight }),
        MatProgressSpinnerModule,
        MatButtonModule,
        MatDialogModule,
        MatMenuModule,
        MatProgressBarModule,
        MatSidenavModule,
        FlexLayoutModule,
        MatRippleModule
    ]
})
export class AppCommonModule {
}
