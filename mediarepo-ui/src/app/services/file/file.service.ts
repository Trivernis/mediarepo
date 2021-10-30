import {Inject, Injectable, Sanitizer} from '@angular/core';
import {BehaviorSubject} from "rxjs";
import {File} from "../../models/File";
import {invoke} from "@tauri-apps/api/tauri";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
import {Thumbnail} from "../../models/Thumbnail";
import {TagQuery} from "../../models/TagQuery";
import {SortKey} from "../../models/SortKey";

@Injectable({
  providedIn: 'root'
})
export class FileService {

  displayedFiles = new BehaviorSubject<File[]>([]);

  constructor(@Inject(DomSanitizer) private sanitizer: DomSanitizer) {

  }

  public async getFiles() {
    let all_files = await invoke<File[]>("plugin:mediarepo|get_all_files");
    this.displayedFiles.next(all_files);
  }

  public async findFiles(tags: TagQuery[], sortBy: SortKey[]) {
    let files = await invoke<File[]>("plugin:mediarepo|find_files", {tags, sortBy: sortBy.map(k => k.toBackendType())});
    this.displayedFiles.next(files);
  }

  public async readFile(file: File): Promise<SafeResourceUrl> {
    const once_uri =  await invoke<string>("plugin:mediarepo|read_file_by_hash", {hash: file.hash, mimeType: file.mime_type});
    return this.sanitizer.bypassSecurityTrustResourceUrl(once_uri);
  }

  public async readThumbnail(thumbnail: Thumbnail): Promise<SafeResourceUrl> {
    let once_uri = await invoke<string>("plugin:mediarepo|read_thumbnail", {hash: thumbnail.hash, mimeType: thumbnail.mime_type });
    return this.sanitizer.bypassSecurityTrustResourceUrl(once_uri);
  }

  public async getThumbnails(hash: string): Promise<Thumbnail[]> {
    return await invoke<Thumbnail[]>("plugin:mediarepo|get_file_thumbnails", {hash});
  }
}
