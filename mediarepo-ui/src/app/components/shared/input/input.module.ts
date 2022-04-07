import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {NativeFileSelectComponent} from "./native-file-select/native-file-select.component";
import {TagInputComponent} from "./tag-input/tag-input.component";
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import {MatFormFieldModule} from "@angular/material/form-field";
import {ReactiveFormsModule} from "@angular/forms";
import {MatInputModule} from "@angular/material/input";
import {NgIconsModule} from "@ng-icons/core";
import {MatFolder, MatInsertDriveFile} from "@ng-icons/material-icons/baseline";
import {MatButtonModule} from "@angular/material/button";
import {FlexModule} from "@angular/flex-layout";
import {FilterInputComponent} from "./filter-input/filter-input.component";


@NgModule({
    declarations: [
        NativeFileSelectComponent,
        TagInputComponent,
        FilterInputComponent
    ],
    exports: [
        NativeFileSelectComponent,
        TagInputComponent,
        FilterInputComponent
    ],
    imports: [
        CommonModule,
        MatAutocompleteModule,
        MatFormFieldModule,
        ReactiveFormsModule,
        MatInputModule,
        NgIconsModule.withIcons({ MatInsertDriveFile, MatFolder }),
        MatButtonModule,
        FlexModule,
    ]
})
export class InputModule {
}
