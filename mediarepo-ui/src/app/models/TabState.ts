import {BehaviorSubject} from "rxjs";
import {TabCategory} from "./TabCategory";
import {FileService} from "../services/file/file.service";
import {File} from "../../api/models/File";
import {SortKey} from "../../api/models/SortKey";
import {debounceTime} from "rxjs/operators";
import {mapNew} from "../../api/models/adaptors";
import {SearchFilters} from "../../api/models/SearchFilters";
import {SortingPreset} from "../../api/models/SortingPreset";

export class TabState {
    public uuid: number;
    public category: TabCategory;
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

    private fileService: FileService;

    constructor(
        uuid: number,
        category: TabCategory,
        fileService: FileService
    ) {
        this.category = category;
        this.uuid = uuid;
        this.fileService = fileService;
        if (this.category === TabCategory.Files) {
            this.filters.pipe(debounceTime(500))
                .subscribe(async () => await this.findFiles());
            this.sortingPreset.pipe(debounceTime(100))
                .subscribe(async () => await this.findFiles());
        }
    }

    public static fromDTO(
        dto: any,
        fileService: FileService
    ): TabState {
        const state = new TabState(
            dto.uuid,
            dto.category,
            fileService
        );

        state.filters.next(new SearchFilters(dto.filters ?? []));
        state.sortingPreset.next(new SortingPreset(dto.sortingPreset));
        state.mode.next(dto.mode ?? "grid");
        state.selectedCD.next(dto.selectedFileHash);
        state.files.next((dto.files ?? []).map(mapNew(File)));

        return state;
    }

    public async findFiles() {
        this.loading.next(true);
        const files = await this.fileService.findFiles(
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

    public getDTO(): any {
        return {
            uuid: this.uuid,
            category: this.category,
            filters: this.filters.value.getFilters(),
            sortingPreset: this.sortingPreset.value.rawData,
            mode: this.mode.value,
            selectedFileHash: this.selectedCD.value,
            files: this.category === TabCategory.Import ? this.files.value.map(
                f => f.rawData) : [],
        };
    }
}
