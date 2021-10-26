import { Injectable } from '@angular/core';
import {invoke} from "@tauri-apps/api/tauri";
import {Tag} from "../../models/Tag";
import {BehaviorSubject, Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class TagService {

  public tags: BehaviorSubject<Tag[]> = new BehaviorSubject<Tag[]>([]);

  constructor() { }

  public async loadTags() {
    const tags = await invoke<Tag[]>("plugin:mediarepo|get_all_tags");
    this.tags.next(tags.map(t => new Tag(t.id, t.name, t.namespace)));
  }

  public async getTagsForFile(hash: string): Promise<Tag[]> {
    const tags =  await invoke<Tag[]>("plugin:mediarepo|get_tags_for_file", {hash});
    return tags.map(t => new Tag(t.id, t.name, t.namespace));
  }
}
