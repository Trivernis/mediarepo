import {Component, Inject, OnInit} from '@angular/core';
import {
  AbstractControl,
  FormControl,
  FormGroup, ValidationErrors,
  Validators
} from "@angular/forms";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {RepositoryService} from "../../../../services/repository/repository.service";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {dialog} from "@tauri-apps/api";
import {Repository} from "../../../../models/Repository";

@Component({
  selector: 'app-add-repository-dialog',
  templateUrl: './add-repository-dialog.component.html',
  styleUrls: ['./add-repository-dialog.component.scss']
})
export class AddRepositoryDialogComponent implements OnInit {

  formGroup = new FormGroup({
    name: new FormControl("My Repository", [Validators.required]),
    repositoryType: new FormControl("local", [Validators.required]),
    path: new FormControl("", [this.validatePath]),
    address: new FormControl("", [this.validateAddress])
  });

  repositories: Repository[] = [];
  onlineStatus = "Unknown";
  localRepoExists = false;

  constructor(
    public repoService: RepositoryService,
    public errorBroker: ErrorBrokerService,
    public dialogRef: MatDialogRef<AddRepositoryDialogComponent>,
    @Inject(MAT_DIALOG_DATA) data: any) {
  }

  ngOnInit(): void {
    this.repoService.repositories.subscribe(repositories => this.repositories = repositories)
  }

  public async checkRepositoryStatus() {
    this.onlineStatus = "Checking...";
    const address = this.formGroup.value.address;
    const running = await this.repoService.checkDaemonRunning(address);
    console.log(running);
    this.onlineStatus = running? "Online" : "Offline";
  }

  public async checkLocalRepoExists() {
    this.localRepoExists = await this.repoService.checkLocalRepositoryExists(this.formGroup.value.path);
  }

  public async initLocalRepository() {
    const path = this.formGroup.value.path;
    await this.repoService.initRepository(path);
    await this.checkLocalRepoExists();
  }

  public async addRepository() {
    let {name, repositoryType, path, address} = this.formGroup.value;
    path = repositoryType === "local"? path : undefined;
    address = repositoryType === "remote"? address : undefined;
    try {
      await this.repoService.addRepository(name, path, address, repositoryType === "local");
      this.dialogRef.close();
    } catch (err) {
      this.errorBroker.showError(err);
    }
  }

  public closeDialog() {
    this.dialogRef.close();
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

    if (this.repositories.find(r => r.name === value)) {
      control?.setErrors({nameAlreadyExists: value});
    }
  }

  validatePath(control: AbstractControl): ValidationErrors | null {
    const repositoryType = control.parent?.get("repositoryType")?.value ?? "local";

    if (repositoryType === "local") {
      return control.value.length > 0? null : {valueRequired: control.value};
    }
    return null;
  }

  validateAddress(control: AbstractControl): ValidationErrors | null {
    const repositoryType = control.parent?.get("repositoryType")?.value ?? "remote";

    if (repositoryType === "remote") {
      const match = /(\d+\.){3}\d+:\d+|\S+:\d+/.test(control.value)
      return match? null : {invalidAddress: control.value};
    }

    return null;
  }
}
