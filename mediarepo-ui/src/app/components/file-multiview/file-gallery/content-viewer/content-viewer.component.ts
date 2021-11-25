import {
  AfterContentInit, AfterViewInit,
  Component,
  Input,
  OnChanges,
  OnDestroy,
  OnInit,
  SimpleChanges, ViewChild
} from '@angular/core';
import {SafeResourceUrl} from "@angular/platform-browser";
import {File} from "../../../../models/File";
import {FileService} from "../../../../services/file/file.service";
import {FileHelper} from "../../../../services/file/file.helper";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {BusyIndicatorComponent} from "../../../busy-indicator/busy-indicator.component";

type ContentType = "image" | "video" | "audio" | "other";

@Component({
  selector: 'app-content-viewer',
  templateUrl: './content-viewer.component.html',
  styleUrls: ['./content-viewer.component.scss']
})
export class ContentViewerComponent implements AfterViewInit, OnChanges, OnDestroy {
  @Input() file!: File;

  public contentUrl: SafeResourceUrl | undefined;
  public blobUrl: SafeResourceUrl | undefined;

  @ViewChild(BusyIndicatorComponent) busyIndicator!: BusyIndicatorComponent;

  constructor(
    private errorBroker: ErrorBrokerService,
    private fileService: FileService
  ) {
  }

  public async ngAfterViewInit() {
    if (["audio", "video"].includes(this.getContentType())) {
      await this.loadBlobUrl();
    } else {
      this.contentUrl = this.fileService.buildContentUrl(this.file);
    }
  }

  public async ngOnChanges(changes:SimpleChanges) {
    if (changes["file"]) {
      if (["audio", "video"].includes(this.getContentType()) && this.busyIndicator) {
        await this.loadBlobUrl();
      } else {
        this.contentUrl = this.fileService.buildContentUrl(this.file);
        this.unloadBlobUrl();
      }
    }
  }

  public ngOnDestroy(): void {
    this.unloadBlobUrl();
  }

  public getContentType(): ContentType {
    if (!this.file.mime_type) {
      return "other";
    }
    let mimeParts = this.file.mime_type.split("/");
    const type = mimeParts.shift() ?? "other";
    const subtype = mimeParts.shift() ?? "*";

    switch (type) {
      case "image":
        return "image";
      case "video":
        return "video";
      case "audio":
        return "audio";
      default:
        return "other";
    }
  }

  public async downloadContent() {
    const path = await FileHelper.getFileDownloadLocation(this.file)

    if (path) {
      try {
        await this.fileService.saveFile(this.file, path);
      } catch (err) {
        this.errorBroker.showError(err);
      }
    }
  }

  public async loadBlobUrl(): Promise<void> {
    await this.busyIndicator.wrapAsyncOperation(async () => {
      const startId = this.file.id;
      this.unloadBlobUrl();
      const url = await this.fileService.readFile(this.file);
      if (startId === this.file.id) {
        this.blobUrl = url;
      }
    });
  }

  private unloadBlobUrl() {
    if (this.blobUrl) {
      URL?.revokeObjectURL(this.blobUrl as string);
      this.blobUrl = undefined;
    }
  }
}
