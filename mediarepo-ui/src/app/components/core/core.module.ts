import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {CoreComponent} from "./core.component";
import {RepositoriesTabComponent} from "./repositories-tab/repositories-tab.component";
import {FilesTabComponent} from "./files-tab/files-tab.component";
import {FilesTabSidebarComponent} from "./files-tab/files-tab-sidebar/files-tab-sidebar.component";
import {ImportTabComponent} from "./import-tab/import-tab.component";
import {ImportTabSidebarComponent} from "./import-tab/import-tab-sidebar/import-tab-sidebar.component";
import {FilesystemImportComponent} from "./import-tab/import-tab-sidebar/filesystem-import/filesystem-import.component";
import {MatButtonModule} from "@angular/material/button";
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatTabsModule} from "@angular/material/tabs";
import {FlexModule} from "@angular/flex-layout";
import {MatOptionModule, MatRippleModule} from "@angular/material/core";
import {MatSelectModule} from "@angular/material/select";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {SharedModule} from "../shared/shared.module";
import {MatDividerModule} from "@angular/material/divider";
import {NgIconsModule} from "@ng-icons/core";
import * as materialIcons from "@ng-icons/material-icons";
import {MatMenuModule} from "@angular/material/menu";


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
    MatMenuModule,
    MatRippleModule,
  ]
})
export class CoreModule {
}
