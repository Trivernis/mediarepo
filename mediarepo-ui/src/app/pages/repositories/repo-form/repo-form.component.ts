import { Component, OnInit } from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {RepositoryService} from "../../../services/repository/repository.service";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-repo-form',
  templateUrl: './repo-form.component.html',
  styleUrls: ['./repo-form.component.scss']
})
export class RepoFormComponent implements OnInit {

  repoForm = new FormGroup({
    name: new FormControl("", Validators.required),
    path: new FormControl("", Validators.required),
  })


  constructor(private repoService: RepositoryService, private snackBar: MatSnackBar) { }

  ngOnInit(): void {
  }

  async addRepository() {
    let {name, path} = this.repoForm.value;
    try {
      await this.repoService.addRepository(name, path);
    } catch(err) {
      this.snackBar.open(err.Msg, undefined, {
        panelClass: "warn"
      })
    }
  }
}
