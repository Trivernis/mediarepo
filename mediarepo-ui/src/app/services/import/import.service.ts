import { Injectable } from '@angular/core';
import {FileOsMetadata} from "../../models/FileOsMetadata";
import {invoke} from "@tauri-apps/api/tauri";

@Injectable({
  providedIn: 'root'
})
export class ImportService {

  constructor() { }

  public async resolvePathsToFiles(paths: string[]): Promise<FileOsMetadata[]> {
    return await invoke<FileOsMetadata[]>("plugin:mediarepo|resolve_paths_to_files", {paths});
  }
}
