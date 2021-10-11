import {Inject, Injectable} from '@angular/core';
import {BehaviorSubject} from "rxjs";
import {File} from "../../models/File";
import {invoke} from "@tauri-apps/api/tauri";
import {DOCUMENT} from "@angular/common";

@Injectable({
  providedIn: 'root'
})
export class FileService {

  displayedFiles = new BehaviorSubject<File[]>([]);

  constructor(@Inject(DOCUMENT) private document: Document) {

  }

  public async getFiles() {
    let all_files = await invoke<File[]>("get_all_files");
    this.displayedFiles.next(all_files.slice(0, 50));
  }

  public async readFile(hash: string, mime_type: string): Promise<string | undefined> {
    const data = await invoke<number[]>("read_file_by_hash", {hash});
    const blob = new Blob([new Uint8Array(data)], {type: mime_type});
    return new Promise<string | undefined>((res, rej) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        const url = e.target?.result
        if (url === null) {
          res(undefined);
        } else {
          res(url as string)
        }
      };
      reader.readAsDataURL(blob);
    })
  }
}
