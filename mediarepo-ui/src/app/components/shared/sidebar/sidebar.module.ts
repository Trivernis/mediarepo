import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {TagEditComponent} from "./tag-edit/tag-edit.component";
import {FileSearchComponent} from "./file-search/file-search.component";
import {NgIconsModule} from "@ng-icons/core";
import {
    MatAddCircle,
    MatChangeCircle,
    MatDeleteSweep,
    MatExpandLess,
    MatExpandMore,
    MatFilterAlt,
    MatRemove,
    MatRemoveCircle,
} from "@ng-icons/material-icons";
import {MatRippleModule} from "@angular/material/core";
import {MatButtonModule} from "@angular/material/button";
import {InputModule} from "../input/input.module";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatDividerModule} from "@angular/material/divider";
import {FlexModule} from "@angular/flex-layout";
import {MatSelectModule} from "@angular/material/select";
import {MatInputModule} from "@angular/material/input";
import {SortDialogComponent} from "./file-search/sort-dialog/sort-dialog.component";
import {FilterDialogComponent} from "./file-search/filter-dialog/filter-dialog.component";
import {MatListModule} from "@angular/material/list";
import {MatDialogModule} from "@angular/material/dialog";
import {AppCommonModule} from "../app-common/app-common.module";
import {DragDropModule} from "@angular/cdk/drag-drop";
import {TagModule} from "../tag/tag.module";
import {FileImportComponent} from "./file-import/file-import.component";
import {FilesystemImportComponent} from "./file-import/filesystem-import/filesystem-import.component";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {MatMenuModule} from "@angular/material/menu";
import {FileMetadataComponent} from "./file-metadata/file-metadata.component";
import {
    EditableMetadataEntryComponent
} from "./file-metadata/editable-metadata-entry/editable-metadata-entry.component";
import {ReactiveFormsModule} from "@angular/forms";
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import {FilterExpressionItemComponent} from "./file-search/filter-expression-item/filter-expression-item.component";
import {TagQueryItemComponent} from "./file-search/filter-expression-item/tag-query-item/tag-query-item.component";
import {
    PropertyQueryItemComponent
} from "./file-search/filter-expression-item/property-query-item/property-query-item.component";
import {
    FilterExpressionListItemComponent
} from "./file-search/filter-dialog/filter-expression-list-item/filter-expression-list-item.component";
import {GetTagQueryPipe} from "./file-search/filter-pipes/get-tag-query.pipe";
import {GetPropertyQueryPipe} from "./file-search/filter-pipes/get-property-query.pipe";
import {SortButtonComponent} from "./file-search/sort-button/sort-button.component";
import {MatTooltipModule} from "@angular/material/tooltip";


@NgModule({
    declarations: [
        TagEditComponent,
        FileSearchComponent,
        SortDialogComponent,
        FilterDialogComponent,
        FileImportComponent,
        FilesystemImportComponent,
        FileMetadataComponent,
        EditableMetadataEntryComponent,
        FilterExpressionItemComponent,
        TagQueryItemComponent,
        PropertyQueryItemComponent,
        FilterExpressionListItemComponent,
        GetTagQueryPipe,
        GetPropertyQueryPipe,
        SortButtonComponent,
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
            MatFilterAlt,
            MatExpandMore,
            MatExpandLess,
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
        MatAutocompleteModule,
        MatTooltipModule,
    ]
})
export class SidebarModule {
}
