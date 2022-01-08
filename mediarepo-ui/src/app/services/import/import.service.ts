import {Injectable} from "@angular/core";
import {AddFileOptions} from "../../models/AddFileOptions";
import {File} from "../../../api/models/File";
import {MediarepApi} from "../../../api/Api";
import {mapNew,} from "../../../api/models/adaptors";
import {FileOsMetadata} from "../../../api/api-types/files";

@Injectable({
    providedIn: "root"
})
export class ImportService {

    constructor() {
    }

    /**
     * Resolves paths from the local file system into a list of files that can be imported
     * @param {string[]} paths
     * @returns {Promise<FileOsMetadata[]>}
     */
    public async resolvePathsToFiles(paths: string[]): Promise<FileOsMetadata[]> {
        return MediarepApi.resolvePathsToFiles({paths});
    }

    /**
     * Imports a file from the local file system
     * @param {FileOsMetadata} metadata
     * @param {AddFileOptions} options
     * @returns {Promise<File>}
     */
    public async addLocalFile(metadata: FileOsMetadata, options: AddFileOptions): Promise<File> {
        return MediarepApi.addLocalFile({metadata, options}).then(mapNew(File));
    }
}
