import {Component} from "@angular/core";
import {RepositoryService} from "../../../services/repository/repository.service";
import {Repository} from "../../../../api/models/Repository";


@Component({
    selector: "app-repositories-tab",
    templateUrl: "./repositories-tab.component.html",
    styleUrls: ["./repositories-tab.component.scss"]
})
export class RepositoriesTabComponent {

    public selectedRepository?: Repository;

    constructor(private repositoryService: RepositoryService) {
        const sub = this.repositoryService.selectedRepository.subscribe(repo => this.selectedRepository = repo);
    }
}
