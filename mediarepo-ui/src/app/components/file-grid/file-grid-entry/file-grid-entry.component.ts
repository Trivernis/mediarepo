import {Component, Input, OnInit} from '@angular/core';
import {File} from "../../../models/File";
import {FileService} from "../../../services/file/file.service";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";

@Component({
  selector: 'app-file-grid-entry',
  templateUrl: './file-grid-entry.component.html',
  styleUrls: ['./file-grid-entry.component.scss']
})
export class FileGridEntryComponent implements OnInit {

  @Input() file: File | undefined;
  contentUrl: string | undefined;
  constructor(private fileService: FileService, private errorBroker: ErrorBrokerService) { }

  async ngOnInit(): Promise<void> {
    if (this.file) {
      console.log(this.file);
      try {
        this.contentUrl = await this.fileService.readFile(this.file.hash, this.file.mime_type ?? "image/png");
      } catch (err) {
        this.errorBroker.showError(err);
      }
    }
  }

}
