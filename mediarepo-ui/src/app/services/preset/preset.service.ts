import {Injectable} from "@angular/core";
import {SortingPreset} from "../../../api/models/SortingPreset";
import {MediarepoApi} from "../../../api/Api";
import {mapMany, mapNew} from "../../../api/models/adaptors";

@Injectable({
    providedIn: "root"
})
export class PresetService {

    constructor() {
    }

    public async getAllSortingPresets(): Promise<SortingPreset[]> {
        return MediarepoApi.getAllSortingPresets().then(mapMany(mapNew(SortingPreset)));
    }
}
