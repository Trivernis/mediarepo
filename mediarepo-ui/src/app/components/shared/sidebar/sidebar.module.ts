import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FileEditComponent} from "./file-edit/file-edit.component";
import {FileSearchComponent} from "./file-search/file-search.component";
import {NgIconsModule} from "@ng-icons/core";
import {MatRemove, MatChangeCircle, MatAddCircle, MatRemoveCircle, MatDeleteSweep, MatFilterAlt} from "@ng-icons/material-icons";
import {MatRippleModule} from "@angular/material/core";
import {MatButtonModule} from "@angular/material/button";
import {InputModule} from "../input/input.module";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatFormFieldModule} from "@angular/material/form-field";
import {SharedModule} from "../shared.module";
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


@NgModule({
    declarations: [
        FileEditComponent,
        FileSearchComponent,
        TagFilterListItemComponent,
        SortDialogComponent,
        FilterDialogComponent,
    ],
    exports: [
        FileEditComponent,
        FileSearchComponent
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
        SharedModule,
        MatDividerModule,
        FlexModule,
        MatSelectModule,
        MatInputModule,
        MatListModule,
        MatDialogModule,
        AppCommonModule,
        DragDropModule,
    ]
})
export class SidebarModule {
}
