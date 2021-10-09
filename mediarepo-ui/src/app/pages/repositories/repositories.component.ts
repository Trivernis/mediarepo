import { Component, OnInit } from '@angular/core';
import {Repository} from "../../models/Repository";
import {RepositoryService} from "../../services/repository/repository.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {FormBuilder, FormGroup} from "@angular/forms";

@Component({
  selector: 'app-repositories',
  templateUrl: './repositories.component.html',
  styleUrls: ['./repositories.component.scss']
})
export class RepositoriesComponent implements OnInit {

  repositories: Repository[] = [];

  constructor(
    private repoService: RepositoryService,
  ) {
  }

  ngOnInit(): void {
    this.repoService.repositories.subscribe({
      next: (repos) => {
        this.repositories = repos;
      }
    });
  }

  async addRepository() {

  }
}
