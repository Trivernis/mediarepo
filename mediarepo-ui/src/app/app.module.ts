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
import {LightboxModule} from "ngx-lightbox";
import {MatChipsModule} from "@angular/material/chips";
import {MatIconModule} from "@angular/material/icon";
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import {FileSearchComponent} from './components/file-search/file-search.component';
import {MatTabsModule} from "@angular/material/tabs";
import {SearchTabComponent} from './pages/home/search-tab/search-tab.component';
import {FlexModule, GridModule} from "@angular/flex-layout";
import {MatRippleModule} from "@angular/material/core";
import {FilterDialogComponent} from './components/file-search/filter-dialog/filter-dialog.component';
import {MatDialogModule} from "@angular/material/dialog";
import {MatSelectModule} from "@angular/material/select";
import { FileGalleryComponent } from './components/file-gallery/file-gallery.component';
import { FileGalleryEntryComponent } from './components/file-gallery/file-gallery-entry/file-gallery-entry.component';
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {BlockUIModule} from "primeng/blockui";
import {PanelModule} from "primeng/panel";
import {DragDropModule} from "@angular/cdk/drag-drop";
import { ContentAwareImageComponent } from './components/content-aware-image/content-aware-image.component';
import {MatSliderModule} from "@angular/material/slider";
import { AddRepositoryDialogComponent } from './pages/home/repositories-tab/add-repository-dialog/add-repository-dialog.component';
import {MatTooltipModule} from "@angular/material/tooltip";

@NgModule({
  declarations: [
    AppComponent,
    RepositoriesTabComponent,
    HomeComponent,
    RepositoryCardComponent,
    FileGridComponent,
    FileGridEntryComponent,
    FileSearchComponent,
    SearchTabComponent,
    FilterDialogComponent,
    FileGalleryComponent,
    FileGalleryEntryComponent,
    ContentAwareImageComponent,
    AddRepositoryDialogComponent,
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
    LightboxModule,
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
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
}
