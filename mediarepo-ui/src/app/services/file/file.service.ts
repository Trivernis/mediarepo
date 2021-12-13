import {Inject, Injectable} from "@angular/core";
import {File} from "../../models/File";
import {invoke} from "@tauri-apps/api/tauri";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
import {SortKey} from "../../models/SortKey";
import {FilterExpression} from "../../models/FilterExpression";


@Injectable({
    providedIn: "root"
})
export class FileService {

    constructor(
        @Inject(DomSanitizer) private sanitizer: DomSanitizer,
    ) {
    }
    public async getAllFiles(): Promise<File[]> {
        return await invoke<File[]>("plugin:mediarepo|get_all_files");
    }

    public async findFiles(filters: FilterExpression[], sortBy: SortKey[]): Promise<File[]> {
        let backendFilters = filters.map(f => f.toBackendType());
        return await invoke<File[]>("plugin:mediarepo|find_files",
            {
                filters: backendFilters,
                sortBy: sortBy.map(k => k.toBackendType())
            });
    }

    public async updateFileName(file: File, name: string): Promise<File> {
        return await invoke<File>("plugin:mediarepo|update_file_name",
            {id: file.id, name})
    }

    /**
     * Builds a safe thumbnail url that accesses custom scheme for thumbnails
     * @param {File} file
     * @param {number} height
     * @param {number} width
     * @returns {SafeResourceUrl}
     */
    public buildThumbnailUrl(file: File, height: number, width: number): SafeResourceUrl {
        return this.sanitizer.bypassSecurityTrustResourceUrl(
            `thumb://${file.hash}?width=${250}&height=${250}`)
    }

    /**
     * Builds a safe content url that accesses custom scheme for thumbnails
     * @param {File} file
     * @returns {SafeResourceUrl}
     */
    public buildContentUrl(file: File): SafeResourceUrl {
        return this.sanitizer.bypassSecurityTrustResourceUrl(
            `content://${file.hash}`)
    }

    /**
     * Saves a file locally
     * @param {File} file
     * @param {string} targetPath
     * @returns {Promise<void>}
     */
    public async saveFile(file: File, targetPath: string) {
        await invoke("plugin:mediarepo|save_file_locally",
            {id: file.id, path: targetPath})
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
        const data = await invoke<number[]>("plugin:mediarepo|read_file",
            {hash: file.hash, mimeType: file.mime_type});
        const blob = new Blob([new Uint8Array(data)], {type: file.mime_type});
        const url = URL?.createObjectURL(blob);
        return this.sanitizer.bypassSecurityTrustResourceUrl(url);
    }
}
