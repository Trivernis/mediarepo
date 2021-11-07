import {Inject, Injectable} from '@angular/core';
import {BehaviorSubject} from "rxjs";
import {File} from "../../models/File";
import {invoke} from "@tauri-apps/api/tauri";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
import {Thumbnail} from "../../models/Thumbnail";
import {TagQuery} from "../../models/TagQuery";
import {SortKey} from "../../models/SortKey";
import {RepositoryService} from "../repository/repository.service";

@Injectable({
  providedIn: 'root'
})
export class FileService {

  displayedFiles = new BehaviorSubject<File[]>([]);
  thumbnailCache: {[key: number]: Thumbnail[]} = {};

  constructor(@Inject(DomSanitizer) private sanitizer: DomSanitizer, private repoService: RepositoryService) {
    repoService.selectedRepository.subscribe(_ => this.clearCache());
  }

  public clearCache() {
    this.thumbnailCache = {};
  }

  public async getFiles() {
    let all_files = await invoke<File[]>("plugin:mediarepo|get_all_files");
    this.displayedFiles.next(all_files);
  }

  public async findFiles(tags: TagQuery[], sortBy: SortKey[]) {
    let files = await invoke<File[]>("plugin:mediarepo|find_files",
      {tags, sortBy: sortBy.map(k => k.toBackendType())});
    this.displayedFiles.next(files);
  }

  public async readFile(file: File): Promise<SafeResourceUrl> {
    const once_uri = await invoke<string>("plugin:mediarepo|read_file_by_hash",
      {id: file.id, hash: file.hash, mimeType: file.mime_type});
    return this.sanitizer.bypassSecurityTrustResourceUrl(once_uri);
  }

  /**
   * Returns the thumbnail for a file with a specific size (allowing +-10%)
   * If none can be found it asks the backend if it has one or generates one of that size
   * @param {File} file
   * @param {number} width
   * @param {number} height
   * @returns {Promise<SafeResourceUrl>}
   */
  public async getFileThumbnail(file: File, width: number, height: number): Promise<SafeResourceUrl> {
    const thumbnails = await this.getThumbnails(file);
    const thumbnail = thumbnails.find(t => t.height >= height * 0.7 && t.width >= width * 0.7 && t.height <= height * 1.3 && t.width <= width * 1.3);
    let url;

    if (thumbnail) {
      url = await this.readThumbnail(thumbnail);
    } else {
      url = await this.getThumbnailOfSize(file, height * 0.9, width * 0.9, height * 1.1, width * 1.1);
      delete this.thumbnailCache[file.id];
    }

    return url;
  }

  public async readThumbnail(thumbnail: Thumbnail): Promise<SafeResourceUrl> {
    let once_uri = await invoke<string>("plugin:mediarepo|read_thumbnail",
      {hash: thumbnail.hash, mimeType: thumbnail.mime_type});
    return this.sanitizer.bypassSecurityTrustResourceUrl(once_uri);
  }

  public async getThumbnailOfSize(file: File, minHeight: number, minWidth: number, maxHeight: number, maxWidth: number): Promise<SafeResourceUrl> {
    let once_uri = await invoke<string>("plugin:mediarepo|get_thumbnail_of_size", {fileId: file.id, minSize: [minHeight, minWidth], maxSize: [maxHeight, maxWidth]});
    return this.sanitizer.bypassSecurityTrustResourceUrl(once_uri);
  }

  public async getThumbnails(file: File): Promise<Thumbnail[]> {
    const cachedThumbnails = this.thumbnailCache[file.id];
    if (cachedThumbnails) {
      return cachedThumbnails;
    }
    const thumbnails = await invoke<Thumbnail[]>("plugin:mediarepo|get_file_thumbnails",
      {id: file.id});
    this.thumbnailCache[file.id] = thumbnails;

    return thumbnails;
  }

  public async updateFileName(file: File, name: string): Promise<File> {
    return await invoke<File>("plugin:mediarepo|update_file_name", {id: file.id, name})
  }
}
