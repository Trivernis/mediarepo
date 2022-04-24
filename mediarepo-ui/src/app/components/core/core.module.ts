import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {CoreComponent} from "./core.component";
import {RepositoriesTabComponent} from "./repositories-tab/repositories-tab.component";
import {FilesTabComponent} from "./files-tab/files-tab.component";
import {FilesTabSidebarComponent} from "./files-tab/files-tab-sidebar/files-tab-sidebar.component";
import {ImportTabComponent} from "./import-tab/import-tab.component";
import {ImportTabSidebarComponent} from "./import-tab/import-tab-sidebar/import-tab-sidebar.component";
import {MatButtonModule} from "@angular/material/button";
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatTabsModule} from "@angular/material/tabs";
import {FlexModule} from "@angular/flex-layout";
import {MatOptionModule, MatRippleModule} from "@angular/material/core";
import {MatSelectModule} from "@angular/material/select";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {MatDividerModule} from "@angular/material/divider";
import {NgIconsModule} from "@ng-icons/core";
import {MatMoreVert, MatPlus} from "@ng-icons/material-icons/baseline";
import {MatMenuModule} from "@angular/material/menu";
import {InputModule} from "../shared/input/input.module";
import {SidebarModule} from "../shared/sidebar/sidebar.module";
import {FileModule} from "../shared/file/file.module";
import {AppCommonModule} from "../shared/app-common/app-common.module";
import {ReactiveFormsModule} from "@angular/forms";
import {RepositoryCardComponent} from "./repositories-tab/repository-card/repository-card.component";
import {MatCardModule} from "@angular/material/card";
import {MatListModule} from "@angular/material/list";
import {MatDialogModule} from "@angular/material/dialog";
import {MatTooltipModule} from "@angular/material/tooltip";
import {MatInputModule} from "@angular/material/input";
import {TagModule} from "../shared/tag/tag.module";
import {
    DownloadDaemonDialogComponent
} from "./repositories-tab/download-daemon-dialog/download-daemon-dialog.component";
import {RepositoryModule} from "../shared/repository/repository.module";
import {MatToolbarModule} from "@angular/material/toolbar";
import {
    RepositoryDetailsViewComponent
} from "./repositories-tab/repository-details-view/repository-details-view.component";
import {EmptyTabComponent} from "./empty-tab/empty-tab.component";
import {RepositoryOverviewComponent} from "./repositories-tab/repository-overview/repository-overview.component";
import {AboutDialogComponent} from "./repositories-tab/repository-overview/about-dialog/about-dialog.component";


@NgModule({
    declarations: [
        RepositoriesTabComponent,
        CoreComponent,
        FilesTabComponent,
        FilesTabSidebarComponent,
        ImportTabComponent,
        ImportTabSidebarComponent,
        RepositoryCardComponent,
        DownloadDaemonDialogComponent,
        RepositoryDetailsViewComponent,
        EmptyTabComponent,
        RepositoryOverviewComponent,
        AboutDialogComponent,
    ],
    exports: [
        CoreComponent,
    ],
    imports: [
        CommonModule,
        MatTabsModule,
        MatSidenavModule,
        MatOptionModule,
        MatSelectModule,
        MatDividerModule,
        MatProgressBarModule,
        MatCheckboxModule,
        ScrollingModule,
        NgIconsModule.withIcons({ MatPlus, MatMoreVert }),
        FlexModule,
        MatButtonModule,
        MatMenuModule,
        MatRippleModule,
        InputModule,
        SidebarModule,
        FileModule,
        AppCommonModule,
        ReactiveFormsModule,
        MatCardModule,
        MatListModule,
        MatDialogModule,
        MatTooltipModule,
        MatInputModule,
        TagModule,
        RepositoryModule,
        MatToolbarModule
    ]
})
export class CoreModule {
}
