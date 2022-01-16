import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FileActionBaseComponent} from "./file-action-base/file-action-base.component";


@NgModule({
    declarations: [
        FileActionBaseComponent,
    ],
    exports: [
        FileActionBaseComponent,
    ],
    imports: [
        CommonModule
    ]
})
export class AppBaseModule {
}
