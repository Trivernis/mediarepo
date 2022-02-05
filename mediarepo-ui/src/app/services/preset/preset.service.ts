import {Injectable} from "@angular/core";
import {SortingPreset} from "../../../api/models/SortingPreset";
import {MediarepoApi} from "../../../api/Api";
import {mapMany, mapNew} from "../../../api/models/adaptors";
import {SortKey} from "../../../api/models/SortKey";

@Injectable({
    providedIn: "root"
})
export class PresetService {

    constructor() {
    }

    public async getAllSortingPresets(): Promise<SortingPreset[]> {
        return MediarepoApi.getAllSortingPresets().then(mapMany(mapNew(SortingPreset)));
    }

    public async addSortingPreset(keys: SortKey[]): Promise<SortingPreset> {
        return MediarepoApi.addSortingPreset({ sortKeys: keys.map(k => k.rawData) }).then(mapNew(SortingPreset));
    }

    public async deleteSortingPreset(id: number): Promise<void> {
        return MediarepoApi.deleteSortingPreset({ id });
    }
}
