import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RepositoryCardComponent} from "../core/repositories-tab/repository-card/repository-card.component";
import {FileGridComponent} from "./file-multiview/file-grid/file-grid.component";
import {FileGridEntryComponent} from "./file-multiview/file-grid/file-grid-entry/file-grid-entry.component";
import {FileSearchComponent} from "./file-search/file-search.component";
import {SortDialogComponent} from "./file-search/sort-dialog/sort-dialog.component";
import {FileGalleryComponent} from "./file-multiview/file-gallery/file-gallery.component";
import {FileGalleryEntryComponent} from "./file-multiview/file-gallery/file-gallery-entry/file-gallery-entry.component";
import {ContentAwareImageComponent} from "./content-aware-image/content-aware-image.component";
import {AddRepositoryDialogComponent} from "../core/repositories-tab/add-repository-dialog/add-repository-dialog.component";
import {ConfirmDialogComponent} from "./confirm-dialog/confirm-dialog.component";
import {TagItemComponent} from "./tag-item/tag-item.component";
import {FileEditComponent} from "./file-edit/file-edit.component";
import {NativeFileSelectComponent} from "./inputs/native-file-select/native-file-select.component";
import {FilterDialogComponent} from "./file-search/filter-dialog/filter-dialog.component";
import {TagFilterListItemComponent} from "./file-search/filter-dialog/tag-filter-list-item/tag-filter-list-item.component";
import {TagInputComponent} from "./inputs/tag-input/tag-input.component";
import {ContextMenuComponent} from "./context-menu/context-menu.component";
import {FileContextMenuComponent} from "./context-menu/file-context-menu/file-context-menu.component";
import {ContentViewerComponent} from "./file-multiview/file-gallery/content-viewer/content-viewer.component";
import {ImageViewerComponent} from "./file-multiview/file-gallery/content-viewer/image-viewer/image-viewer.component";
import {VideoViewerComponent} from "./file-multiview/file-gallery/content-viewer/video-viewer/video-viewer.component";
import {AudioViewerComponent} from "./file-multiview/file-gallery/content-viewer/audio-viewer/audio-viewer.component";
import {BusyIndicatorComponent} from "./busy-indicator/busy-indicator.component";
import {FileThumbnailComponent} from "./file-thumbnail/file-thumbnail.component";
import {FileMultiviewComponent} from "./file-multiview/file-multiview.component";
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


@NgModule({
  declarations: [
    RepositoryCardComponent,
    FileGridComponent,
    FileGridEntryComponent,
    FileSearchComponent,
    SortDialogComponent,
    FileGalleryComponent,
    FileGalleryEntryComponent,
    ContentAwareImageComponent,
    AddRepositoryDialogComponent,
    ConfirmDialogComponent,
    TagItemComponent,
    FileEditComponent,
    NativeFileSelectComponent,
    FilterDialogComponent,
    TagFilterListItemComponent,
    TagInputComponent,
    ContextMenuComponent,
    FileContextMenuComponent,
    ContentViewerComponent,
    ImageViewerComponent,
    VideoViewerComponent,
    AudioViewerComponent,
    BusyIndicatorComponent,
    FileThumbnailComponent,
    FileMultiviewComponent,
  ],
  exports: [
    FileMultiviewComponent,
    NativeFileSelectComponent,
    BusyIndicatorComponent,
    FileSearchComponent,
    TagItemComponent,
    FileEditComponent,
    ContextMenuComponent,
    RepositoryCardComponent
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
  ]
})
export class SharedModule {
}
