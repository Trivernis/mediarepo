import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CoreComponent} from "./components/core/core.component";

const routes: Routes = [
    {path: "", component: CoreComponent}];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})
export class AppRoutingModule {
}
