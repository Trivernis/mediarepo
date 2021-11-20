import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {RepositoriesTabComponent} from './pages/home/repositories-tab/repositories-tab.component';
import {HomeComponent} from './pages/home/home.component';
import {RepositoryCardComponent} from './pages/home/repositories-tab/repository-card/repository-card.component';
import {MatCardModule} from "@angular/material/card";
import {MatListModule} from "@angular/material/list";
import {MatButtonModule} from "@angular/material/button";
import {MatToolbarModule} from "@angular/material/toolbar";
import {MatSnackBarModule} from "@angular/material/snack-bar";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {ReactiveFormsModule} from "@angular/forms";
import {FileGridComponent} from './components/file-grid/file-grid.component';
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatGridListModule} from "@angular/material/grid-list";
import {FileGridEntryComponent} from './components/file-grid/file-grid-entry/file-grid-entry.component';
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {MatPaginatorModule} from "@angular/material/paginator";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatChipsModule} from "@angular/material/chips";
import {MatIconModule} from "@angular/material/icon";
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import {FileSearchComponent} from './components/file-search/file-search.component';
import {MatTabsModule} from "@angular/material/tabs";
import {FilesTabComponent} from './pages/home/files-tab/files-tab.component';
import {FlexModule, GridModule} from "@angular/flex-layout";
import {MatRippleModule} from "@angular/material/core";
import {SortDialogComponent} from './components/file-search/sort-dialog/sort-dialog.component';
import {MatDialogModule} from "@angular/material/dialog";
import {MatSelectModule} from "@angular/material/select";
import {FileGalleryComponent} from './components/file-gallery/file-gallery.component';
import {FileGalleryEntryComponent} from './components/file-gallery/file-gallery-entry/file-gallery-entry.component';
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {BlockUIModule} from "primeng/blockui";
import {PanelModule} from "primeng/panel";
import {DragDropModule} from "@angular/cdk/drag-drop";
import {ContentAwareImageComponent} from './components/content-aware-image/content-aware-image.component';
import {MatSliderModule} from "@angular/material/slider";
import {AddRepositoryDialogComponent} from './pages/home/repositories-tab/add-repository-dialog/add-repository-dialog.component';
import {MatTooltipModule} from "@angular/material/tooltip";
import {MatMenuModule} from "@angular/material/menu";
import {ConfirmDialogComponent} from './components/confirm-dialog/confirm-dialog.component';
import {FilesTabSidebarComponent} from './pages/home/files-tab/files-tab-sidebar/files-tab-sidebar.component';
import {MatExpansionModule} from "@angular/material/expansion";
import {TagItemComponent} from './components/tag-item/tag-item.component';
import { FileEditComponent } from './components/file-edit/file-edit.component';
import { ImportTabComponent } from './pages/home/import-tab/import-tab.component';
import { ImportTabSidebarComponent } from './pages/home/import-tab/import-tab-sidebar/import-tab-sidebar.component';
import { NativeFileSelectComponent } from './components/inputs/native-file-select/native-file-select.component';
import { FilesystemImportComponent } from './pages/home/import-tab/import-tab-sidebar/filesystem-import/filesystem-import.component';
import {MatCheckboxModule} from "@angular/material/checkbox";
import { FilterDialogComponent } from './components/file-search/filter-dialog/filter-dialog.component';
import { TagFilterListItemComponent } from './components/file-search/filter-dialog/tag-filter-list-item/tag-filter-list-item.component';
import { TagInputComponent } from './components/inputs/tag-input/tag-input.component';

@NgModule({
  declarations: [
    AppComponent,
    RepositoriesTabComponent,
    HomeComponent,
    RepositoryCardComponent,
    FileGridComponent,
    FileGridEntryComponent,
    FileSearchComponent,
    FilesTabComponent,
    SortDialogComponent,
    FileGalleryComponent,
    FileGalleryEntryComponent,
    ContentAwareImageComponent,
    AddRepositoryDialogComponent,
    ConfirmDialogComponent,
    FilesTabSidebarComponent,
    TagItemComponent,
    FileEditComponent,
    ImportTabComponent,
    ImportTabSidebarComponent,
    NativeFileSelectComponent,
    FilesystemImportComponent,
    FilterDialogComponent,
    TagFilterListItemComponent,
    TagInputComponent,
  ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        BrowserAnimationsModule,
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
        MatIconModule,
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
    ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
}
