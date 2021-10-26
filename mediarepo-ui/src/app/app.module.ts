import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { RepositoriesComponent } from './pages/repositories/repositories.component';
import { HomeComponent } from './pages/home/home.component';
import { RepositoryCardComponent } from './pages/repositories/repository-card/repository-card.component';
import {MatCardModule} from "@angular/material/card";
import {MatListModule} from "@angular/material/list";
import {MatButtonModule} from "@angular/material/button";
import {MatToolbarModule} from "@angular/material/toolbar";
import {MatSnackBarModule} from "@angular/material/snack-bar";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {ReactiveFormsModule} from "@angular/forms";
import { RepoFormComponent } from './pages/repositories/repo-form/repo-form.component';
import { FileGridComponent } from './components/file-grid/file-grid.component';
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatGridListModule} from "@angular/material/grid-list";
import { FileGridEntryComponent } from './components/file-grid/file-grid-entry/file-grid-entry.component';
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {MatPaginatorModule} from "@angular/material/paginator";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {LightboxModule} from "ngx-lightbox";
import {MatChipsModule} from "@angular/material/chips";
import {MatIconModule} from "@angular/material/icon";
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import { FileSearchComponent } from './components/file-search/file-search.component';

@NgModule({
  declarations: [
    AppComponent,
    RepositoriesComponent,
    HomeComponent,
    RepositoryCardComponent,
    RepoFormComponent,
    FileGridComponent,
    FileGridEntryComponent,
    FileSearchComponent,
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
        MatAutocompleteModule
    ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
