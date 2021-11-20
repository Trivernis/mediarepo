import {Component, ViewChild} from '@angular/core';
import {File} from "../../../models/File";
import {ContextMenuComponent} from "../context-menu.component";
import {clipboard, dialog} from "@tauri-apps/api";
import {FileService} from "../../../services/file/file.service";
import {ErrorBrokerService} from "../../../services/error-broker/error-broker.service";
import {downloadDir} from "@tauri-apps/api/path";

@Component({
  selector: 'app-file-context-menu',
  templateUrl: './file-context-menu.component.html',
  styleUrls: ['./file-context-menu.component.scss']
})
export class FileContextMenuComponent {

  public file!: File;

  @ViewChild("contextMenu") contextMenu!: ContextMenuComponent;

  constructor(private fileService: FileService, private errorBroker: ErrorBrokerService) { }

  public onContextMenu(event: MouseEvent, file: File) {
    this.file = file;
    this.contextMenu.onContextMenu(event);
  }

  public async copyFileHash(): Promise<void> {
    await clipboard.writeText(this.file.hash);
  }

  public async exportFile(): Promise<void> {
    let extension;
    if (this.file.mime_type) {
      extension = FileContextMenuComponent.getExtensionForMime(this.file.mime_type);
    }
    const downloadDirectory = await downloadDir();
    const suggestionPath = downloadDirectory + this.file.hash + "." + extension;

    const path = await dialog.save({
      defaultPath: suggestionPath,
      filters: [{name: this.file.mime_type ?? "All", extensions: [extension ?? "*"]}, {name: "All", extensions: ["*"]}]
    });
    if (path) {
      try {
        await this.fileService.saveFile(this.file, path);
      } catch (err) {
        this.errorBroker.showError(err);
      }
    }
  }

  /**
   * Returns the extension for a mime type
   * @param {string} mime
   * @returns {string | undefined}
   * @private
   */
  private static getExtensionForMime(mime: string): string | undefined {
    let parts = mime.split("/");

    if (parts.length === 2) {
      const type = parts[0];
      const subtype = parts[1];
      return FileContextMenuComponent.convertMimeSubtypeToExtension(subtype);
    }
    return undefined;
  }

  private static convertMimeSubtypeToExtension(subtype: string): string {
    return subtype;
  }
}
