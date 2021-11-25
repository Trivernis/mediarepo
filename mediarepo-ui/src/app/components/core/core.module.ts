import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {CoreComponent} from "./core.component";
import {RepositoriesTabComponent} from "./repositories-tab/repositories-tab.component";
import {FilesTabComponent} from "./files-tab/files-tab.component";
import {FilesTabSidebarComponent} from "./files-tab/files-tab-sidebar/files-tab-sidebar.component";
import {ImportTabComponent} from "./import-tab/import-tab.component";
import {ImportTabSidebarComponent} from "./import-tab/import-tab-sidebar/import-tab-sidebar.component";
import {FilesystemImportComponent} from "./import-tab/import-tab-sidebar/filesystem-import/filesystem-import.component";
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
import {MatOptionModule, MatRippleModule} from "@angular/material/core";
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
import {SharedModule} from "../shared/shared.module";
import {MatDividerModule} from "@angular/material/divider";
import {NgIconsModule} from "@ng-icons/core";
import * as materialIcons from "@ng-icons/material-icons";


@NgModule({
  declarations: [
    RepositoriesTabComponent,
    CoreComponent,
    FilesTabComponent,
    FilesTabSidebarComponent,
    ImportTabComponent,
    ImportTabSidebarComponent,
    FilesystemImportComponent,
  ],
  exports: [
    CoreComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    MatTabsModule,
    MatSidenavModule,
    MatOptionModule,
    MatSelectModule,
    MatDividerModule,
    MatProgressBarModule,
    MatCheckboxModule,
    ScrollingModule,
    NgIconsModule.withIcons({...materialIcons}),
    FlexModule,
    MatButtonModule,
  ]
})
export class CoreModule {
}
