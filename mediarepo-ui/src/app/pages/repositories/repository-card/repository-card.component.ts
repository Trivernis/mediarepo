import {Component, Input, OnInit} from '@angular/core';
import {Repository} from "../../../models/Repository";
import {RepositoryService} from "../../../services/repository/repository.service";
import {Router} from "@angular/router";

@Component({
  selector: 'app-repository-card',
  templateUrl: './repository-card.component.html',
  styleUrls: ['./repository-card.component.scss']
})
export class RepositoryCardComponent implements OnInit {

  @Input() repository?: Repository;

  constructor(private repoService: RepositoryService, private router: Router) {}

  ngOnInit(): void {
  }

  async selectRepository() {
    if (this.repository) {
      this.repoService.setRepository(this.repository);
      await this.router.navigate([""]);
    }
  }
}
