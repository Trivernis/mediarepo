import {TabSaveState, TabState} from "./TabState";
import {SaveState} from "./SaveState";
import {StateServices} from "./StateServices";
import {TabCategory} from "./TabCategory";
import {BehaviorSubject} from "rxjs";
import {File} from "../../../api/models/File";
import {FileBasicData} from "../../../api/api-types/files";
import {mapNew} from "../../../api/models/adaptors";

export type ImportTabSaveState = {
    selectedCd: string | undefined,
    mode: "grid" | "gallery",
    files: FileBasicData[],
} & TabSaveState;

export class ImportTabState extends TabState implements SaveState<ImportTabSaveState> {

    public mode = new BehaviorSubject<"grid" | "gallery">("grid");
    public selectedCD = new BehaviorSubject<string | undefined>(undefined);
    public files = new BehaviorSubject<File[]>([]);

    constructor(uuid: number, services: StateServices) {
        super(uuid, TabCategory.Import, services);
    }

    public restoreSaveState(state: ImportTabSaveState) {
        super.restoreSaveState(state);
        this.selectedCD = new BehaviorSubject<string | undefined>(state.selectedCd);
        this.files = new BehaviorSubject<File[]>(state.files.map(mapNew(File)));
        this.mode = new BehaviorSubject<"grid" | "gallery">(state.mode);

        return self;
    }

    public toSaveState(): ImportTabSaveState {
        return {
            uuid: this.uuid,
            category: this.category,
            selectedCd: this.selectedCD.value,
            files: this.files.value.map(f => f.rawData),
            mode: this.mode.value
        };
    }
}
