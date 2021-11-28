import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {TagEditComponent} from "./tag-edit/tag-edit.component";
import {FileSearchComponent} from "./file-search/file-search.component";
import {NgIconsModule} from "@ng-icons/core";
import {MatRemove, MatChangeCircle, MatAddCircle, MatRemoveCircle, MatDeleteSweep, MatFilterAlt, MatSave, MatEdit} from "@ng-icons/material-icons";
import {MatRippleModule} from "@angular/material/core";
import {MatButtonModule} from "@angular/material/button";
import {InputModule} from "../input/input.module";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatDividerModule} from "@angular/material/divider";
import {FlexModule} from "@angular/flex-layout";
import {MatSelectModule} from "@angular/material/select";
import {MatInputModule} from "@angular/material/input";
import {TagFilterListItemComponent} from "./file-search/filter-dialog/tag-filter-list-item/tag-filter-list-item.component";
import {SortDialogComponent} from "./file-search/sort-dialog/sort-dialog.component";
import {FilterDialogComponent} from "./file-search/filter-dialog/filter-dialog.component";
import {MatListModule} from "@angular/material/list";
import {MatDialogModule} from "@angular/material/dialog";
import {AppCommonModule} from "../app-common/app-common.module";
import {DragDropModule} from "@angular/cdk/drag-drop";
import {TagModule} from "../tag/tag.module";
import { FileImportComponent } from "./file-import/file-import.component";
import {FilesystemImportComponent} from "./file-import/filesystem-import/filesystem-import.component";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {MatMenuModule} from "@angular/material/menu";
import { FileMetadataComponent } from "./file-metadata/file-metadata.component";
import { MetadataEntryComponent } from "./file-metadata/metadata-entry/metadata-entry.component";
import { EditableMetadataEntryComponent } from "./file-metadata/editable-metadata-entry/editable-metadata-entry.component";
import {ReactiveFormsModule} from "@angular/forms";


@NgModule({
    declarations: [
        TagEditComponent,
        FileSearchComponent,
        TagFilterListItemComponent,
        SortDialogComponent,
        FilterDialogComponent,
        FileImportComponent,
        FilesystemImportComponent,
        FileMetadataComponent,
        MetadataEntryComponent,
        EditableMetadataEntryComponent,
    ],
    exports: [
        TagEditComponent,
        FileSearchComponent,
        FileImportComponent,
        FileMetadataComponent
    ],
    imports: [
        CommonModule,
        NgIconsModule.withIcons({
            MatRemove,
            MatChangeCircle,
            MatAddCircle,
            MatRemoveCircle,
            MatDeleteSweep,
            MatFilterAlt
        }),
        MatRippleModule,
        MatButtonModule,
        InputModule,
        ScrollingModule,
        MatFormFieldModule,
        MatDividerModule,
        FlexModule,
        MatSelectModule,
        MatInputModule,
        MatListModule,
        MatDialogModule,
        AppCommonModule,
        DragDropModule,
        TagModule,
        MatCheckboxModule,
        MatProgressBarModule,
        MatMenuModule,
        ReactiveFormsModule,
    ]
})
export class SidebarModule {
}
