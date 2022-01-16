import {Component, Inject, ViewChild} from "@angular/core";
import {
    RepositoryFormComponent
} from "../repository-form/repository-form.component";
import {
    RepositoryService
} from "../../../../../services/repository/repository.service";
import {
    ErrorBrokerService
} from "../../../../../services/error-broker/error-broker.service";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {Repository} from "../../../../../../api/models/Repository";

@Component({
    selector: "app-edit-repository-dialog",
    templateUrl: "./edit-repository-dialog.component.html",
    styleUrls: ["./edit-repository-dialog.component.scss"]
})
export class EditRepositoryDialogComponent {

    @ViewChild(RepositoryFormComponent) repositoryForm!: RepositoryFormComponent;

    public selectedRepository: Repository;
    public originalName: string;

    constructor(
        public repoService: RepositoryService,
        public errorBroker: ErrorBrokerService,
        public dialogRef: MatDialogRef<EditRepositoryDialogComponent>,
        @Inject(MAT_DIALOG_DATA) data: any) {
        this.selectedRepository = data.repository;
        this.originalName = this.selectedRepository.name;
    }

    public async checkLocalRepoExists() {
        this.repositoryForm.localRepoExists = await this.repoService.checkLocalRepositoryExists(
            this.repositoryForm.formGroup.value.path);
    }

    public async initLocalRepository() {
        const path = this.repositoryForm.formGroup.value.path;
        try {
            await this.repoService.initRepository(path);
        } catch (err) {
            this.errorBroker.showError(err);
        }
        await this.checkLocalRepoExists();
    }

    public async addRepository() {
        let {name, repositoryType, path, address} = this.repositoryForm.formGroup.value;
        path = repositoryType === "local" ? path : undefined;
        address = repositoryType === "remote" ? address : undefined;

        if (this.originalName === this.repoService.selectedRepository.getValue()?.name) {
            await this.repoService.closeSelectedRepository();
        }

        try {
            if (name != this.originalName) {
                await this.repoService.removeRepository(this.originalName);
            }
            await this.repoService.addRepository(name, path, address,
                repositoryType === "local");
            this.selectedRepository.update({name, local: repositoryType === "local", path, address});

            this.dialogRef.close();
        } catch (err) {
            this.errorBroker.showError(err);
        }
    }

    public closeDialog() {
        this.dialogRef.close();
    }
}
