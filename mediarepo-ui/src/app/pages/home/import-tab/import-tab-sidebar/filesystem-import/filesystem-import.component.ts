import {Component} from '@angular/core';
import {FileOsMetadata} from "../../../../../models/FileOsMetadata";
import {ImportService} from "../../../../../services/import/import.service";
import {ErrorBrokerService} from "../../../../../services/error-broker/error-broker.service";

@Component({
  selector: 'app-filesystem-import',
  templateUrl: './filesystem-import.component.html',
  styleUrls: ['./filesystem-import.component.scss']
})
export class FilesystemImportComponent {

  public fileCount: number = 0;
  public files: FileOsMetadata[] = [];
  public importTagsFromTxt = true;
  public deleteAfterImport = false;

  public resolving = false;

  constructor(private errorBroker: ErrorBrokerService, private importService: ImportService) {
  }

  public async setSelectedPaths(paths: string[]) {
    this.resolving = true;
    try {
      this.files = await this.importService.resolvePathsToFiles(paths);
      this.fileCount = this.files.length;
    } catch (err) {
      console.log(err);
      this.errorBroker.showError(err);
    }
    this.resolving = false;
  }
}
