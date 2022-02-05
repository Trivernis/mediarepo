import {Inject, Injectable} from "@angular/core";
import {File} from "../../../api/models/File";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
import {SortKey} from "../../../api/models/SortKey";
import {MediarepoApi} from "../../../api/Api";
import {mapMany, mapNew} from "../../../api/models/adaptors";
import {FileMetadata, FileStatus} from "../../../api/api-types/files";
import {SearchFilters} from "../../../api/models/SearchFilters";


@Injectable({
    providedIn: "root"
})
export class FileService {

    constructor(
        @Inject(DomSanitizer) private sanitizer: DomSanitizer,
    ) {
    }

    public async getAllFiles(): Promise<File[]> {
        return MediarepoApi.getAllFiles().then(mapMany(mapNew(File)));
    }

    public async findFiles(filters: SearchFilters, sortBy: SortKey[]): Promise<File[]> {
        return MediarepoApi.findFiles(
            {
                filters: filters.getFilters(),
                sortBy: sortBy.map(k => k.rawData)
            })
            .then(mapMany(mapNew(File)));
    }

    /**
     * Returns metadata about a file
     * @param {number} id
     * @returns {Promise<FileMetadata>}
     */
    public async getFileMetadata(id: number): Promise<FileMetadata> {
        return MediarepoApi.getFileMetadata({ id });
    }

    /**
     * Updates the filename of a file
     * @param {number} id
     * @param {string} name
     * @returns {Promise<FileMetadata>}
     */
    public async updateFileName(id: number, name: string): Promise<FileMetadata> {
        return MediarepoApi.updateFileName({ id, name });
    }

    /**
     * Updates the status of a file
     * @param {number} id
     * @param {FileStatus} status
     * @returns {Promise<File>}
     */
    public async updateFileStatus(id: number, status: FileStatus): Promise<File> {
        return MediarepoApi.updateFileStatus({ id, status }).then(mapNew(File));
    }

    /***
     * Permanently deletes a file
     * @param {number} id
     * @returns {Promise<void>}
     */
    public async deleteFile(id: number): Promise<void> {
        return MediarepoApi.deleteFile({ id });
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
            `thumb://${file.cd}?width=${250}&height=${250}`);
    }

    /**
     * Builds a safe content url that accesses custom scheme for thumbnails
     * @param {File} file
     * @returns {SafeResourceUrl}
     */
    public buildContentUrl(file: File): SafeResourceUrl {
        return this.sanitizer.bypassSecurityTrustResourceUrl(
            `content://${file.cd}`);
    }

    /**
     * Saves a file locally
     * @param {File} file
     * @param {string} targetPath
     * @returns {Promise<void>}
     */
    public async saveFile(file: File, targetPath: string) {
        await MediarepoApi.saveFileLocally({ id: file.id, path: targetPath });
    }

    /**
     * Deletes all thumbnails of a file
     * @param {File} file
     * @returns {Promise<void>}
     */
    public async deleteThumbnails(file: File) {
        await MediarepoApi.deleteThumbnails({ id: file.id });
    }

    /**
     * Reads the contents of a file and returns the object url for it
     * @param {File} file
     * @returns {Promise<SafeResourceUrl>}
     */
    public async readFile(file: File): Promise<SafeResourceUrl> {
        const data = await MediarepoApi.readFile(
            { mimeType: file.mimeType, hash: file.cd });
        const blob = new Blob([new Uint8Array(data)], { type: file.mimeType });
        const url = URL?.createObjectURL(blob);
        return this.sanitizer.bypassSecurityTrustResourceUrl(url);
    }
}
