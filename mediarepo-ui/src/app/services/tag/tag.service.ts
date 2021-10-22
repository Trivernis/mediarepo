import { Injectable } from '@angular/core';
import {invoke} from "@tauri-apps/api/tauri";
import {Tag} from "../../models/Tag";

@Injectable({
  providedIn: 'root'
})
export class TagService {

  constructor() { }

  public async getTagsForFile(hash: string): Promise<Tag[]> {
    return await invoke<Tag[]>("get_tags_for_file", {hash});
  }
}
