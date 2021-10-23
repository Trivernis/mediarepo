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

  public async findFiles(tags: string[]) {
    let files = await invoke<File[]>("find_files", {tags});
    this.displayedFiles.next(files);
  }

  public async readFile(hash: string, mime: string): Promise<SafeResourceUrl> {
    const once_uri =  await invoke<string>("read_file_by_hash", {hash, mime});
    return this.sanitizer.bypassSecurityTrustResourceUrl(once_uri);
  }

  public async readThumbnail(thumbnail: Thumbnail): Promise<SafeResourceUrl> {
    let once_uri = await invoke<string>("read_thumbnail", {hash: thumbnail.hash, mime: thumbnail.mime});
    return this.sanitizer.bypassSecurityTrustResourceUrl(once_uri);
  }

  public async getThumbnails(hash: string): Promise<Thumbnail[]> {
    return await invoke<Thumbnail[]>("get_thumbnails", {hash});
  }

  createSafeObjectUrl(blob: Blob): SafeResourceUrl {
    const url = URL?.createObjectURL(blob);
    return this.sanitizer.bypassSecurityTrustResourceUrl(url);
  }
}
