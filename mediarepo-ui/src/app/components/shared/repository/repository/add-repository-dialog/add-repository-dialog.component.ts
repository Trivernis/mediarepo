import {Component, Inject, ViewChild} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {
    RepositoryFormComponent
} from "../repository-form/repository-form.component";
import {
    RepositoryService
} from "../../../../../services/repository/repository.service";
import {
    ErrorBrokerService
} from "../../../../../services/error-broker/error-broker.service";

@Component({
    selector: "app-add-repository-dialog",
    templateUrl: "./add-repository-dialog.component.html",
    styleUrls: ["./add-repository-dialog.component.scss"]
})
export class AddRepositoryDialogComponent {

    @ViewChild(RepositoryFormComponent) repositoryForm!: RepositoryFormComponent;

    constructor(
        public repoService: RepositoryService,
        public errorBroker: ErrorBrokerService,
        public dialogRef: MatDialogRef<AddRepositoryDialogComponent>,
        @Inject(MAT_DIALOG_DATA) data: any) {
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
        try {
            await this.repoService.addRepository(name, path, address,
                repositoryType === "local");
            this.dialogRef.close();
        } catch (err) {
            this.errorBroker.showError(err);
        }
    }

    public closeDialog() {
        this.dialogRef.close();
    }
}
