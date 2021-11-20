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

  public async saveFile(file: File, targetPath: string) {
    await invoke("plugin:mediarepo|save_file_locally", {id: file.id, path: targetPath})
  }
}
