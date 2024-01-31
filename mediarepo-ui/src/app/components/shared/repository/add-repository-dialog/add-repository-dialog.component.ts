import {Component, Inject, ViewChild} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {RepositoryFormComponent} from "../repository-form/repository-form.component";
import {RepositoryService} from "../../../../services/repository/repository.service";
import {LoggingService} from "../../../../services/logging/logging.service";

@Component({
    selector: "app-add-repository-dialog",
    templateUrl: "./add-repository-dialog.component.html",
    styleUrls: ["./add-repository-dialog.component.scss"]
})
export class AddRepositoryDialogComponent {

    @ViewChild(RepositoryFormComponent) repositoryForm!: RepositoryFormComponent;

    constructor(
        public repoService: RepositoryService,
        public errorBroker: LoggingService,
        public dialogRef: MatDialogRef<AddRepositoryDialogComponent>,
        @Inject(MAT_DIALOG_DATA) data: any
    ) {
    }

    public async checkLocalRepoExists() {
        this.repositoryForm.localRepoExists = await this.repoService.checkLocalRepositoryExists(
            this.repositoryForm.formGroup.value.path as unknown as string);
    }

    public async initLocalRepository() {
        const path = this.repositoryForm.formGroup.value.path as unknown as string;
        try {
            await this.repoService.initRepository(path);
        } catch (err: any) {
            this.errorBroker.error(err);
        }
        await this.checkLocalRepoExists();
    }

    public async addRepository() {
        let { name, repositoryType, path, address } = this.repositoryForm.formGroup.value as unknown as any;
        path = repositoryType === "local" ? path : undefined;
        address = repositoryType === "remote" ? address : undefined;
        try {
            await this.repoService.addRepository(name, path, address,
                repositoryType === "local"
            );
            this.dialogRef.close();
        } catch (err: any) {
            this.errorBroker.error(err);
        }
    }

    public closeDialog() {
        this.dialogRef.close();
    }
}
