import {SortKey} from "./SortKey";
import {SortingPresetData} from "../api-types/presets";
import {mapNew} from "./adaptors";

export class SortingPreset {
    private keys: SortKey[];

    constructor(presetData: SortingPresetData) {
        this._id = presetData.id;
        this.keys = presetData.keys.map(mapNew(SortKey));
    }

    private _id: number;

    public get id(): number {
        return this._id;
    }

    public set id(value: number) {
        this._id = value;
    }

    public get sortKeys(): SortKey[] {
        return this.keys;
    }

    public get rawData(): SortingPresetData {
        return {
            id: this._id,
            keys: this.keys.map(k => k.rawData),
        };
    }

    public static fromValues(id: number, keys: SortKey[]) {
        let preset = new SortingPreset({ id, keys: [] });
        preset.keys = keys;
        return preset;
    }
}
