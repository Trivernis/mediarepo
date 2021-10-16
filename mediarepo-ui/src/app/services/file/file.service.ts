import {Inject, Injectable, Sanitizer} from '@angular/core';
import {BehaviorSubject} from "rxjs";
import {File} from "../../models/File";
import {invoke} from "@tauri-apps/api/tauri";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
import {Thumbnail} from "../../models/Thumbnail";

@Injectable({
  providedIn: 'root'
})
export class FileService {

  displayedFiles = new BehaviorSubject<File[]>([]);

  constructor(@Inject(DomSanitizer) private sanitizer: DomSanitizer) {

  }

  public async getFiles() {
    let all_files = await invoke<File[]>("get_all_files");
    this.displayedFiles.next(all_files);
  }

  public async readFile(hash: string, mime_type: string): Promise<SafeResourceUrl> {
    const data = await invoke<number[]>("read_file_by_hash", {hash});
    const blob = new Blob([new Uint8Array(data)], {type: mime_type});

    return this.createSafeObjectUrl(blob);
  }

  public async readThumbnail(thumbnail: Thumbnail): Promise<SafeResourceUrl> {
    let data = await invoke<number[]>("read_thumbnail", {hash: thumbnail.hash});
    const blob = new Blob([new Uint8Array(data)], {type: thumbnail.mime});

    return this.createSafeObjectUrl(blob);
  }

  public async getThumbnails(hash: string): Promise<Thumbnail[]> {
    return await invoke<Thumbnail[]>("get_thumbnails", {hash});
  }

  createSafeObjectUrl(blob: Blob): SafeResourceUrl {
    const url = URL?.createObjectURL(blob);
    return this.sanitizer.bypassSecurityTrustResourceUrl(url);
  }
}
