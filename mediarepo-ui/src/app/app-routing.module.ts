import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {RepositoriesComponent} from "./pages/repositories/repositories.component";
import {HomeComponent} from "./pages/home/home.component";

const routes: Routes = [
  {path: "repositories", component: RepositoriesComponent},
  {path: "", component: HomeComponent}];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
