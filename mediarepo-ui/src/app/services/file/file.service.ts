import {Inject, Injectable} from '@angular/core';
import {BehaviorSubject} from "rxjs";
import {File} from "../../models/File";
import {invoke} from "@tauri-apps/api/tauri";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
import {Thumbnail} from "../../models/Thumbnail";
import {TagQuery} from "../../models/TagQuery";
import {SortKey} from "../../models/SortKey";
import {RepositoryService} from "../repository/repository.service";
import {FilterExpression} from "../../models/FilterExpression";
import {HttpClient} from "@angular/common/http";
import {map} from "rxjs/operators";
import {http} from "@tauri-apps/api";

@Injectable({
  providedIn: 'root'
})
export class FileService {

  displayedFiles = new BehaviorSubject<File[]>([]);
  thumbnailCache: {[key: number]: Thumbnail[]} = {};

  constructor(
    @Inject(DomSanitizer) private sanitizer: DomSanitizer,
    private repoService: RepositoryService,
    private http: HttpClient,
  ) {
    repoService.selectedRepository.subscribe(_ => this.clearCache());
  }

  public clearCache() {
    this.thumbnailCache = {};
  }

  public async getFiles() {
    let all_files = await invoke<File[]>("plugin:mediarepo|get_all_files");
    this.displayedFiles.next(all_files);
  }

  public async findFiles(filters: FilterExpression[], sortBy: SortKey[]) {
    console.log(filters);
    let files = await invoke<File[]>("plugin:mediarepo|find_files",
      {filters, sortBy: sortBy.map(k => k.toBackendType())});
    this.displayedFiles.next(files);
  }

  public async updateFileName(file: File, name: string): Promise<File> {
    return await invoke<File>("plugin:mediarepo|update_file_name", {id: file.id, name})
  }

  /**
   * Builds a safe thumbnail url that accesses custom scheme for thumbnails
   * @param {File} file
   * @param {number} height
   * @param {number} width
   * @returns {SafeResourceUrl}
   */
  public buildThumbnailUrl(file: File, height: number, width: number): SafeResourceUrl {
    return this.sanitizer.bypassSecurityTrustResourceUrl(`thumb://${file.hash}?width=${250}&height=${250}`)
  }

  /**
   * Builds a safe content url that accesses custom scheme for thumbnails
   * @param {File} file
   * @returns {SafeResourceUrl}
   */
  public buildContentUrl(file: File): SafeResourceUrl {
    return this.sanitizer.bypassSecurityTrustResourceUrl(`content://${file.hash}`)
  }

  /**
   * Saves a file locally
   * @param {File} file
   * @param {string} targetPath
   * @returns {Promise<void>}
   */
  public async saveFile(file: File, targetPath: string) {
    await invoke("plugin:mediarepo|save_file_locally", {id: file.id, path: targetPath})
  }

  /**
   * Deletes all thumbnails of a file
   * @param {File} file
   * @returns {Promise<void>}
   */
  public async deleteThumbnails(file: File) {
    await invoke("plugin:mediarepo|delete_thumbnails", {id: file.id});
  }

  /**
   * Reads the contents of a file and returns the object url for it
   * @param {File} file
   * @returns {Promise<SafeResourceUrl>}
   */
  public async readFile(file: File): Promise<SafeResourceUrl> {
    const data = await invoke<number[]>("plugin:mediarepo|read_file", {hash: file.hash, mimeType: file.mime_type});
    const blob = new Blob([new Uint8Array(data)], {type: file.mime_type});
    const url = URL?.createObjectURL(blob);
    return this.sanitizer.bypassSecurityTrustResourceUrl(url);
  }
}
