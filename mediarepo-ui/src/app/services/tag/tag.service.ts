import {Injectable} from "@angular/core";
import {Tag} from "../../../api/models/Tag";
import {BehaviorSubject} from "rxjs";
import {Namespace} from "../../../api/models/Namespace";
import {mapMany, mapNew} from "../../../api/models/adaptors";
import {MediarepApi} from "../../../api/Api";

@Injectable({
    providedIn: "root"
})
export class TagService {

    public tags: BehaviorSubject<Tag[]> = new BehaviorSubject<Tag[]>([]);
    public namespaces: BehaviorSubject<Namespace[]> = new BehaviorSubject<Namespace[]>([]);

    constructor() {
    }

    public async loadTags() {
        const tags = await MediarepApi.getAllTags().then(mapMany(mapNew(Tag)));
        this.tags.next(tags);
    }

    public async loadNamespaces() {
        const namespaces = await MediarepApi.getAllNamespaces().then(mapMany(mapNew(Namespace)));
        this.namespaces.next(namespaces);
    }

    public async getTagsForFiles(cds: string[]): Promise<Tag[]> {
        let tags: Tag[] = [];
        if (cds.length > 0) {
            tags = await MediarepApi.getTagsForFiles({cds}).then(mapMany(mapNew(Tag)));
        }
        return tags;
    }

    public async createTags(tags: string[]): Promise<Tag[]> {
        return MediarepApi.createTags({tags}).then(mapMany(mapNew(Tag)));
    }

    public async changeFileTags(fileId: number, addedTags: number[], removedTags: number[]): Promise<Tag[]> {
        return MediarepApi.changeFileTags({id: fileId, addedTags, removedTags}).then(mapMany(mapNew(Tag)));
    }
}
