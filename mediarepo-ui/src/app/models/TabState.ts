import {BehaviorSubject} from "rxjs";
import {TabCategory} from "./TabCategory";
import {FileService} from "../services/file/file.service";
import {File} from "../../api/models/File";
import {
    GenericFilter,
    OrFilterExpression,
    SingleFilterExpression
} from "./GenericFilter";
import {SortKey} from "./SortKey";
import {TagQuery} from "./TagQuery";
import {debounceTime} from "rxjs/operators";
import {mapNew} from "../../api/models/adaptors";

export class TabState {
    public uuid: number;
    public category: TabCategory;
    public mode = new BehaviorSubject<"grid" | "gallery">("grid");
    public selectedCD = new BehaviorSubject<string | undefined>(undefined);
    public loading = new BehaviorSubject<boolean>(false);

    public files = new BehaviorSubject<File[]>([]);
    public filters = new BehaviorSubject<GenericFilter[]>([]);
    public sortKeys = new BehaviorSubject<SortKey[]>(
        [new SortKey("FileImportedTime",
            "Ascending", undefined)]);

    private fileService: FileService;

    constructor(uuid: number, category: TabCategory, fileService: FileService) {
        this.category = category;
        this.uuid = uuid;
        this.fileService = fileService;
        if (this.category === TabCategory.Files) {
            this.filters.pipe(debounceTime(500))
                .subscribe(async () => await this.findFiles());
            this.sortKeys.pipe(debounceTime(100))
                .subscribe(async () => await this.findFiles());
        }
    }

    public async findFiles() {
        this.loading.next(true);
        const files = await this.fileService.findFiles(this.filters.value,
            this.sortKeys.value);
        this.files.next(files);
        this.loading.next(false);
    }

    public setFilters(filters: GenericFilter[]) {
        this.filters.next(filters);
    }

    public setSortKeys(keys: SortKey[]) {
        this.sortKeys.next(keys);
    }

    public static fromDTO(dto: any, fileService: FileService): TabState {
        const state = new TabState(dto.uuid, dto.category, fileService);
        const filters = dto.filters.map((f: {filter: any, filter_type: any}) => {
            if (f.filter_type === "OrExpression") {
                return new OrFilterExpression(f.filter.map((f: any) => new TagQuery(f.tag, f.negate)));
            } else {
                return new SingleFilterExpression(new TagQuery(f.filter.tag, f.filter.negate));
            }
        });
        const sortKeys = dto.sortKeys.map((s: {sortType: any, sortDirection: any, namespaceName: any}) =>
            new SortKey(s.sortType, s.sortDirection, s.namespaceName)
        );
        state.filters.next(filters);
        state.sortKeys.next(sortKeys);
        state.mode.next(dto.mode ?? "grid");
        state.selectedCD.next(dto.selectedFileHash);
        state.files.next((dto.files ?? []).map(mapNew(File)));

        return state;
    }

    public getDTO(): any {
        return {
            uuid: this.uuid,
            category: this.category,
            filters: this.filters.value,
            sortKeys: this.sortKeys.value,
            mode: this.mode.value,
            selectedFileHash: this.selectedCD.value,
            files: this.category === TabCategory.Import? this.files.value.map(f => f.rawData) : [],
        };
    }
}
