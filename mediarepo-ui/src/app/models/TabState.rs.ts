import {BehaviorSubject} from "rxjs";
import {TabCategory} from "./TabCategory";
import {FileService} from "../services/file/file.service";
import {File} from "./File";
import {FilterExpression} from "./FilterExpression";
import {SortKey} from "./SortKey";

export class TabState {
    public uuid: number;
    public category: TabCategory;
    public files = new BehaviorSubject<File[]>([]);

    private fileService: FileService;

    constructor(uuid: number, category: TabCategory, fileService: FileService) {
        this.category = category;
        this.uuid = uuid;
        this.fileService = fileService;
    }

    public async loadAllFiles() {
        const files = await this.fileService.getAllFiles();
        this.files.next(files);
    }

    public async findFiles(filters: FilterExpression[], sortBy: SortKey[]) {
        const files = await this.fileService.findFiles(filters, sortBy);
        this.files.next(files);
    }
}
