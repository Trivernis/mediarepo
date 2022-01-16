import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {TagItemComponent} from "./tag-item/tag-item.component";


@NgModule({
    declarations: [TagItemComponent],
    exports: [TagItemComponent],
    imports: [
        CommonModule
    ]
})
export class TagModule {
}
