import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {TagItemComponent} from "./tag-item/tag-item.component";
import {MatCardModule} from "@angular/material/card";
import {MatListModule} from "@angular/material/list";
import {MatButtonModule} from "@angular/material/button";
import {MatToolbarModule} from "@angular/material/toolbar";
import {MatSnackBarModule} from "@angular/material/snack-bar";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {ReactiveFormsModule} from "@angular/forms";
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatGridListModule} from "@angular/material/grid-list";
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {MatPaginatorModule} from "@angular/material/paginator";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatChipsModule} from "@angular/material/chips";
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import {MatTabsModule} from "@angular/material/tabs";
import {FlexModule, GridModule} from "@angular/flex-layout";
import {MatRippleModule} from "@angular/material/core";
import {MatDialogModule} from "@angular/material/dialog";
import {MatSelectModule} from "@angular/material/select";
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {BlockUIModule} from "primeng/blockui";
import {PanelModule} from "primeng/panel";
import {DragDropModule} from "@angular/cdk/drag-drop";
import {MatSliderModule} from "@angular/material/slider";
import {MatTooltipModule} from "@angular/material/tooltip";
import {MatMenuModule} from "@angular/material/menu";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {AppRoutingModule} from "../../app-routing.module";
import {NgIconsModule} from "@ng-icons/core";
import * as materialIcons from "@ng-icons/material-icons";
import {InputModule} from "./input/input.module";
import {AppCommonModule} from "./app-common/app-common.module";


@NgModule({
    declarations: [
        TagItemComponent,
    ],
    exports: [
        TagItemComponent,
    ],
    imports: [
        CommonModule,
        MatCardModule,
        MatListModule,
        MatButtonModule,
        MatToolbarModule,
        MatSnackBarModule,
        MatFormFieldModule,
        MatInputModule,
        ReactiveFormsModule,
        MatSidenavModule,
        MatGridListModule,
        MatProgressBarModule,
        MatPaginatorModule,
        ScrollingModule,
        MatChipsModule,
        MatAutocompleteModule,
        MatTabsModule,
        FlexModule,
        GridModule,
        MatRippleModule,
        MatDialogModule,
        MatSelectModule,
        MatProgressSpinnerModule,
        BlockUIModule,
        PanelModule,
        DragDropModule,
        MatSliderModule,
        MatTooltipModule,
        MatMenuModule,
        MatExpansionModule,
        MatCheckboxModule,
        AppRoutingModule,
        NgIconsModule.withIcons({...materialIcons}),
        InputModule,
        AppCommonModule,
    ]
})
export class SharedModule {
}
