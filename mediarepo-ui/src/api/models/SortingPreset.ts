import {SortKey} from "./SortKey";
import {SortingPresetData} from "../api-types/presets";

export class SortingPreset {
    private readonly _id: number;
    private keys: SortKey[];

    constructor(presetData: SortingPresetData) {
        this._id = presetData.id;
        this.keys = presetData.keys.map(SortKey.fromRawData);
    }

    public get id(): number {
        return this._id;
    }

    public get sortKeys(): SortKey[] {
        return this.sortKeys;
    }
}
