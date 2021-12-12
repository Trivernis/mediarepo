import {BehaviorSubject} from "rxjs";
import {TabCategory} from "./TabCategory";
import {FileService} from "../services/file/file.service";
import {File} from "./File";
import {FilterExpression} from "./FilterExpression";
import {SortKey} from "./SortKey";
import {debounceTime} from "rxjs/operators";

export class TabState {
    public uuid: number;
    public category: TabCategory;
    public files = new BehaviorSubject<File[]>([]);
    public filters = new BehaviorSubject<FilterExpression[]>([]);
    public sortKeys = new BehaviorSubject<SortKey[]>(
        [new SortKey("FileImportedTime",
            "Ascending", undefined)]);

    private fileService: FileService;

    constructor(uuid: number, category: TabCategory, fileService: FileService) {
        this.category = category;
        this.uuid = uuid;
        this.fileService = fileService;
        this.filters.pipe(debounceTime(500))
            .subscribe(async () => await this.findFiles());
        this.sortKeys.pipe(debounceTime(100))
            .subscribe(async () => await this.findFiles());
    }

    public async findFiles() {
        const files = await this.fileService.findFiles(this.filters.value,
            this.sortKeys.value);
        this.files.next(files);
    }

    public setFilters(filters: FilterExpression[]) {
        this.filters.next(filters);
    }

    public setSortKeys(keys: SortKey[]) {
        this.sortKeys.next(keys)
    }
}
