import {Injectable} from "@angular/core";
import {FileOsMetadata} from "../../models/FileOsMetadata";
import {invoke} from "@tauri-apps/api/tauri";
import {AddFileOptions} from "../../models/AddFileOptions";
import {File} from "../../models/File";

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
        return await invoke<FileOsMetadata[]>(
            "plugin:mediarepo|resolve_paths_to_files", {paths});
    }

    /**
     * Imports a file from the local file system
     * @param {FileOsMetadata} metadata
     * @param {AddFileOptions} options
     * @returns {Promise<File>}
     */
    public async addLocalFile(metadata: FileOsMetadata, options: AddFileOptions): Promise<File> {
        return await invoke<File>("plugin:mediarepo|add_local_file",
            {metadata, options});
    }
}
