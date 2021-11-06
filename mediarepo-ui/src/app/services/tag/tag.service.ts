import {Injectable} from '@angular/core';
import {invoke} from "@tauri-apps/api/tauri";
import {Tag} from "../../models/Tag";
import {BehaviorSubject} from "rxjs";
import {File} from "../../models/File";

@Injectable({
  providedIn: 'root'
})
export class TagService {

  public tags: BehaviorSubject<Tag[]> = new BehaviorSubject<Tag[]>([]);

  constructor() {
  }

  public async loadTags() {
    const tags = await invoke<Tag[]>("plugin:mediarepo|get_all_tags");
    this.tags.next(tags.map(t => new Tag(t.id, t.name, t.namespace)));
  }

  public async getTagsForFiles(hashes: string[]): Promise<Tag[]> {
    let tags: Tag[] = []
    if (hashes.length > 0) {
      tags = await invoke<Tag[]>("plugin:mediarepo|get_tags_for_files",
        {hashes});
    }
    return tags.map(t => new Tag(t.id, t.name, t.namespace));
  }

  public async createTags(tags: string[]): Promise<Tag[]> {
    const resultTags = await invoke<Tag[]>("plugin:mediarepo|create_tags", {tags});
    return resultTags.map(t => new Tag(t.id, t.name, t.namespace));
  }

  public async changeFileTags(fileId: number, addedTags: number[], removedTags: number[]): Promise<Tag[]> {
    const tags = await invoke<Tag[]>("plugin:mediarepo|change_file_tags", {id: fileId, addedTags, removedTags});
    return tags.map(t => new Tag(t.id, t.name, t.namespace));
  }
}
