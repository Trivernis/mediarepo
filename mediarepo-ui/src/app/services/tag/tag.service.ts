import {Injectable} from "@angular/core";
import {Tag} from "../../../api/models/Tag";
import {BehaviorSubject} from "rxjs";
import {Namespace} from "../../../api/models/Namespace";
import {mapMany, mapNew} from "../../../api/models/adaptors";
import {MediarepoApi} from "../../../api/Api";

@Injectable({
    providedIn: "root"
})
export class TagService {

    public tags: BehaviorSubject<Tag[]> = new BehaviorSubject<Tag[]>([]);
    public namespaces: BehaviorSubject<Namespace[]> = new BehaviorSubject<Namespace[]>([]);

    constructor() {
    }

    public async loadTags() {
        const tags = await MediarepoApi.getAllTags().then(mapMany(mapNew(Tag)));
        this.tags.next(tags);
    }

    public async loadNamespaces() {
        const namespaces = await MediarepoApi.getAllNamespaces().then(mapMany(mapNew(Namespace)));
        this.namespaces.next(namespaces);
    }

    public async getTagsForFiles(cds: string[]): Promise<Tag[]> {
        let tags: Tag[] = [];
        if (cds.length > 0) {
            tags = await MediarepoApi.getTagsForFiles({ cds }).then(mapMany(mapNew(Tag)));
        }
        return tags;
    }

    public async getFileTagMappings(cds: string[]): Promise<{ [key: string]: Tag[] }> {
        if (cds.length > 0) {
            return await MediarepoApi.getFileTagMap({ cds }).then((cdMappings) => {
                let mappings: { [key: string]: Tag[] } = {};
                console.log("TAG MAPPINGS", cdMappings);
                for (const key in cdMappings) {
                    mappings[key] = cdMappings[key].map(mapNew(Tag));
                }
                return mappings;
            });
        } else {
            return {};
        }
    }

    public async createTags(tags: string[]): Promise<Tag[]> {
        return MediarepoApi.createTags({ tags }).then(mapMany(mapNew(Tag)));
    }

    public async changeFileTags(fileId: number, addedTags: number[], removedTags: number[]): Promise<Tag[]> {
        return MediarepoApi.changeFileTags({ id: fileId, addedTags, removedTags }).then(mapMany(mapNew(Tag)));
    }
}
