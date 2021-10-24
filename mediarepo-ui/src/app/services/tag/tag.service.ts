import { Injectable } from '@angular/core';
import {invoke} from "@tauri-apps/api/tauri";
import {Tag} from "../../models/Tag";

@Injectable({
  providedIn: 'root'
})
export class TagService {

  constructor() { }

  public async getTagsForFile(hash: string): Promise<Tag[]> {
    const tags =  await invoke<Tag[]>("plugin:mediarepo|get_tags_for_file", {hash});
    return tags.map(t => new Tag(t.id, t.name, t.namespace));
  }
}
