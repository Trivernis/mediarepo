import {Component, Input, OnInit, Output} from "@angular/core";
import {AbstractControl, FormControl, FormGroup, ValidationErrors, Validators} from "@angular/forms";
import {Repository} from "../../../../../api/models/Repository";
import {RepositoryService} from "../../../../services/repository/repository.service";
import {dialog} from "@tauri-apps/api";
import {MatDialog} from "@angular/material/dialog";

@Component({
    selector: "app-repository-form",
    templateUrl: "./repository-form.component.html",
    styleUrls: ["./repository-form.component.scss"]
})
export class RepositoryFormComponent implements OnInit {

    @Input() name: string = "My RepositoryData";
    @Input() repositoryType: "local" | "remote" = "local";
    @Input() path: string = "";
    @Input() address: string = "";
    @Input() validateNameDuplicate: boolean = false;

    @Output() formGroup = new FormGroup({
        name: new FormControl(this.name, [Validators.required]),
        repositoryType: new FormControl(this.repositoryType, [Validators.required]),
        path: new FormControl(this.path, [this.validatePath]),
        address: new FormControl(this.address, [this.validateAddress])
    });

    onlineStatus = "Unknown";
    localRepoExists = false;

    repositories: Repository[] = [];

    constructor(public repoService: RepositoryService, public dialog: MatDialog) {

    }

    ngOnInit(): void {
        this.repoService.repositories.subscribe(
            repositories => this.repositories = repositories);
        this.formGroup.setValue({
            name: this.name,
            repositoryType: this.repositoryType,
            path: this.path,
            address: this.address
        });
    }

    public async checkRepositoryStatus() {
        this.onlineStatus = "Checking...";
        const address = this.formGroup.value.address;
        const running = await this.repoService.checkDaemonRunning(address);
        console.log(running);
        this.onlineStatus = running ? "Online" : "Offline";
    }

    public async checkLocalRepoExists() {
        this.localRepoExists = await this.repoService.checkLocalRepositoryExists(
            this.formGroup.value.path);
    }

    public async openFolderDialog() {
        const path = await dialog.open({
            directory: true,
            multiple: false,
        });
        this.formGroup.get("path")?.setValue(path);
        await this.checkLocalRepoExists();
    }

    public async onTypeChange(type: string) {
        setTimeout(() => {
            const path = this.formGroup.get("path");
            const address = this.formGroup.get("address");
            switch (type) {
                case "local":
                    address?.clearValidators();
                    address?.setErrors(null);
                    path?.setValidators(this.validatePath);
                    path?.setErrors(this.validatePath(path));
                    break;
                case "remote":
                    path?.clearValidators();
                    path?.setErrors(null);
                    address?.setValidators(this.validateAddress);
                    address?.setErrors(this.validateAddress(address));
                    break;
            }
        }, 0);
    }

    validateName() {
        const control = this.formGroup.get("name");
        const value = control?.value;

        if (this.validateNameDuplicate && this.repositories.find(r => r.name === value)) {
            control?.setErrors({ nameAlreadyExists: value });
        }
    }

    validatePath(control: AbstractControl): ValidationErrors | null {
        const repositoryType = control.parent?.get(
            "repositoryType")?.value ?? "local";

        if (repositoryType === "local") {
            return control.value.length > 0 ? null : { valueRequired: control.value };
        }
        return null;
    }

    validateAddress(control: AbstractControl): ValidationErrors | null {
        const repositoryType = control.parent?.get(
            "repositoryType")?.value ?? "remote";

        if (repositoryType === "remote") {
            const match = /(\d+\.){3}\d+:\d+|\S+:\d+/.test(control.value);
            return match ? null : { invalidAddress: control.value };
        }

        return null;
    }
}
