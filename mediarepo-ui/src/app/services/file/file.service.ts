import {Inject, Injectable} from "@angular/core";
import {File} from "../../../api/models/File";
import {DomSanitizer, SafeResourceUrl} from "@angular/platform-browser";
import {SortKey} from "../../models/SortKey";
import {GenericFilter} from "../../models/GenericFilter";
import {MediarepApi} from "../../../api/Api";
import {mapMany, mapNew} from "../../../api/models/adaptors";
import {FileMetadata} from "../../../api/api-types/files";


@Injectable({
    providedIn: "root"
})
export class FileService {

    constructor(
        @Inject(DomSanitizer) private sanitizer: DomSanitizer,
    ) {
    }

    public async getAllFiles(): Promise<File[]> {
        return MediarepApi.getAllFiles().then(mapMany(mapNew(File)));
    }

    public async findFiles(filters: GenericFilter[], sortBy: SortKey[]): Promise<File[]> {
        let backendFilters = filters.map(f => f.toBackendType());
        return MediarepApi.findFiles({filters: backendFilters, sortBy: sortBy.map(k => k.toBackendType())}).then(mapMany(mapNew(File)));
    }

    public async getFileMetadata(id: number): Promise<FileMetadata> {
        return MediarepApi.getFileMetadata({id});
    }

    public async updateFileName(id: number, name: string): Promise<FileMetadata> {
        return MediarepApi.updateFileName({id, name});
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
        await MediarepApi.saveFileLocally({id: file.id, path: targetPath});
    }

    /**
     * Deletes all thumbnails of a file
     * @param {File} file
     * @returns {Promise<void>}
     */
    public async deleteThumbnails(file: File) {
        await MediarepApi.deleteThumbnails({id: file.id});
    }

    /**
     * Reads the contents of a file and returns the object url for it
     * @param {File} file
     * @returns {Promise<SafeResourceUrl>}
     */
    public async readFile(file: File): Promise<SafeResourceUrl> {
        const data = await MediarepApi.readFile({mimeType: file.mimeType, hash: file.cd});
        const blob = new Blob([new Uint8Array(data)], {type: file.mimeType});
        const url = URL?.createObjectURL(blob);
        return this.sanitizer.bypassSecurityTrustResourceUrl(url);
    }
}
