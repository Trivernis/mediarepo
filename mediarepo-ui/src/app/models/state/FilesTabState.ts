import {BehaviorSubject} from "rxjs";
import {TabCategory} from "./TabCategory";
import {File} from "../../../api/models/File";
import {SortKey} from "../../../api/models/SortKey";
import {debounceTime} from "rxjs/operators";
import {mapNew} from "../../../api/models/adaptors";
import {SearchFilters} from "../../../api/models/SearchFilters";
import {SortingPreset} from "../../../api/models/SortingPreset";
import {TabSaveState, TabState} from "./TabState";
import {StateServices} from "./StateServices";
import {FileBasicData, FilterExpression} from "../../../api/api-types/files";
import {SortingPresetData} from "../../../api/api-types/presets";
import {SaveState} from "./SaveState";

export type FilesTabSaveState = {
    mode: "gallery" | "grid",
    selectedCd: string | undefined,
    files: FileBasicData[],
    filters: FilterExpression[],
    sortingPreset: SortingPresetData,
} & TabSaveState;

export class FilesTabState extends TabState implements SaveState<FilesTabSaveState> {

    public mode = new BehaviorSubject<"grid" | "gallery">("grid");
    public selectedCD = new BehaviorSubject<string | undefined>(undefined);
    public loading = new BehaviorSubject<boolean>(false);

    public files = new BehaviorSubject<File[]>([]);
    public filters = new BehaviorSubject<SearchFilters>(new SearchFilters([]));
    public sortingPreset = new BehaviorSubject<SortingPreset>(SortingPreset.fromValues(
        -1,
        [SortKey.fromValues(
            "FileImportedTime",
            "Ascending",
            undefined
        )]
    ));

    constructor(
        uuid: number,
        services: StateServices,
    ) {
        super(uuid, TabCategory.Files, services);
        this.subscribe();
    }

    public restoreSaveState(
        state: FilesTabSaveState,
    ) {
        super.restoreSaveState(state);
        this.filters = new BehaviorSubject(new SearchFilters(state.filters ?? []));
        this.sortingPreset = new BehaviorSubject(new SortingPreset(state.sortingPreset));
        this.mode = new BehaviorSubject(state.mode ?? "grid");
        this.selectedCD = new BehaviorSubject(state.selectedCd);
        this.files = new BehaviorSubject((state.files ?? []).map(mapNew(File)));
        this.subscribe();
    }

    public async findFiles() {
        this.loading.next(true);
        const files = await this.services.fileService.findFiles(
            this.filters.value,
            this.sortingPreset.value.sortKeys
        );
        this.files.next(files);
        this.loading.next(false);
    }

    public setTagFilters(filters: SearchFilters) {
        this.filters.next(filters);
    }

    public setSortingPreset(preset: SortingPreset) {
        this.sortingPreset.next(preset);
    }

    public toSaveState(): FilesTabSaveState {
        return {
            uuid: this.uuid,
            category: this.category,
            filters: this.filters.value.getFilters(),
            sortingPreset: this.sortingPreset.value.rawData,
            mode: this.mode.value,
            selectedCd: this.selectedCD.value,
            files: this.category === TabCategory.Import ? this.files.value.map(
                f => f.rawData) : [],
        };
    }

    private subscribe() {
        this.filters.pipe(debounceTime(500))
            .subscribe(async () => await this.findFiles());
        this.sortingPreset.pipe(debounceTime(100))
            .subscribe(async () => await this.findFiles());
    }
}
