import {
    AfterViewChecked,
    ChangeDetectionStrategy,
    Component,
    ElementRef,
    EventEmitter,
    Input,
    OnInit,
    Output,
    ViewChild
} from "@angular/core";
import {MatDialog} from "@angular/material/dialog";
import {SortDialogComponent} from "./sort-dialog/sort-dialog.component";
import {LoggingService} from "../../../../services/logging/logging.service";
import {FilterDialogComponent} from "./filter-dialog/filter-dialog.component";
import {Tag} from "../../../../../api/models/Tag";
import {clipboard} from "@tauri-apps/api";
import {FilesTabState} from "../../../../models/state/FilesTabState";
import {FilterQueryBuilder} from "../../../../../api/models/FilterQueryBuilder";
import {SearchFilters} from "../../../../../api/models/SearchFilters";
import {FileStatus, FilterExpression,} from "../../../../../api/api-types/files";
import {filterExpressionToString} from "../../../../utils/filter-utils";
import {MatCheckboxChange} from "@angular/material/checkbox";
import * as deepEqual from "fast-deep-equal";
import {SortingPreset} from "../../../../../api/models/SortingPreset";


@Component({
    selector: "app-file-search",
    templateUrl: "./file-search.component.html",
    styleUrls: ["./file-search.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FileSearchComponent implements AfterViewChecked, OnInit {
    public sortingPreset: SortingPreset = new SortingPreset({ id: -1, keys: [] });
    public filters: SearchFilters = new SearchFilters([]);

    @Input() availableTags: Tag[] = [];
    @Input() contextTags: Tag[] = [];
    @Input() state!: FilesTabState;
    @Input() tagsLoading = false;

    @Output() searchStartEvent = new EventEmitter<void>();
    @Output() searchEndEvent = new EventEmitter<void>();

    @ViewChild("tagInputList") inputList!: ElementRef;

    public contextMenuTag: Tag | undefined;
    public contextMenuFilter: FilterExpression | undefined = undefined;
    public initialFilterInputValue: string | undefined;
    public displayedFilters: FilterExpression[] = [];

    public displayImported = true;
    public displayArchived = true;
    public displayDeleted = false;
    public searchDuration = 0;

    private needsScroll = false;
    private searchStart = Date.now();

    constructor(
        private logger: LoggingService,
        public dialog: MatDialog
    ) {
        this.assignDisplayedFilters();
    }

    public async ngOnInit() {
        this.state.filters.subscribe(f => {
            this.filters = f;
            this.assignDisplayedFilters();
        });
        this.state.sortingPreset.subscribe(s => this.sortingPreset = s);
        this.state.loading.subscribe(l => {
            if (l) {
                this.searchStart = Date.now();
            } else {
                this.searchDuration = Math.round((Date.now() - this.searchStart) / 100) / 10;
            }
        });
        this.applyStatusFromFilters();
        this.needsScroll = true;
        this.assignDisplayedFilters();
    }

    public ngAfterViewChecked(): void {
        if (this.needsScroll) {
            this.inputList.nativeElement.scrollLeft = this.inputList.nativeElement.scrollWidth;
            this.needsScroll = false;
        }
    }

    public async searchForFiles() {
        this.searchStartEvent.emit();
        try {
            await this.state.findFiles();
        } catch (err: any) {
            this.logger.error(err);
        }
        this.searchEndEvent.emit();
    }

    public addFilterExpression(filter: FilterExpression) {
        this.filters.removeFilter(filter);
        this.filters.addFilterExpression(filter);

        this.state.setTagFilters(this.filters);
        this.needsScroll = true;
    }

    public addTagFilter(filterString: string) {
        const filter = FilterQueryBuilder.buildFilterFromString(filterString);
        if (filter) {
            this.addFilterExpression({ Query: filter });
        }
    }

    public getValidSearchTags(): Tag[] {
        return this.availableTags.filter(t => !this.filters.hasFilter({
            Query: FilterQueryBuilder.tag(
                t.getNormalizedOutput(),
                false
            )
        }));
    }

    public async removeAllSearchTags() {
        this.filters = new SearchFilters([]);
        this.state.setTagFilters(this.filters);
        this.updateStatusFilters();
    }

    public async removeFilterExpression(expr: FilterExpression) {
        this.filters.removeFilter(expr);
        this.state.setTagFilters(this.filters);
        this.needsScroll = true;
    }

    public openSortDialog() {
        const sortingPreset = new SortingPreset(JSON.parse(JSON.stringify(this.sortingPreset.rawData)));
        const openedDialog = this.dialog.open(SortDialogComponent, {
            minWidth: "40vw",
            data: {
                sortingPreset,
            },
            disableClose: true,
        });
        openedDialog.afterClosed().subscribe(async (sortingPreset) => {
            if (sortingPreset) {
                this.sortingPreset = sortingPreset;
                this.state.setSortingPreset(this.sortingPreset);
            }
        });
    }

    public openFilterDialog(): void {
        const filterEntries = new SearchFilters(JSON.parse(JSON.stringify(this.filters.getFilters())));

        const filterDialog = this.dialog.open(FilterDialogComponent, {
            minWidth: "25vw",
            maxHeight: "80vh",
            data: {
                filters: filterEntries,
                availableTags: this.availableTags,
            },
            disableClose: true,
        });
        filterDialog.afterClosed().subscribe(async (filterExpression) => {
            if (filterExpression !== undefined || filterExpression?.length > 0) {
                this.filters = filterExpression;
                this.applyStatusFromFilters();
                this.state.setTagFilters(this.filters);
                this.needsScroll = true;
            }
        });
    }

    public async copyToClipboard(text: string) {
        await clipboard.writeText(text);
    }

    public addFilterToInput(param: FilterExpression): void {
        this.initialFilterInputValue = filterExpressionToString(param);
    }

    public setDisplayDeleted(event: MatCheckboxChange) {
        this.displayDeleted = event.checked;
        this.updateStatusFilters();
    }

    public setDisplayArchived(event: MatCheckboxChange) {
        this.displayArchived = event.checked;
        this.updateStatusFilters();
    }

    public setDisplayImported(event: MatCheckboxChange) {
        this.displayImported = event.checked;
        this.updateStatusFilters();
    }

    public isTagFilter(filter: FilterExpression): boolean {
        const tagFilter = this.buildFilterForDisplayProperty();
        return deepEqual(tagFilter, filter);
    }

    public trackByTagId(index: number, item: Tag) {
        return item.id;
    }

    private assignDisplayedFilters() {
        this.displayedFilters = this.filters.getFilters().filter(f => !this.isTagFilter(f));
    }

    private applyStatusFromFilters() {
        const filterImported = FilterQueryBuilder.status("Imported");
        const filterArchived = FilterQueryBuilder.status("Archived");
        const filterDeleted = FilterQueryBuilder.status("Deleted");
        this.displayImported = this.filters.hasSubfilter(filterImported);
        this.displayArchived = this.filters.hasSubfilter(filterArchived);
        this.displayDeleted = this.filters.hasSubfilter(filterDeleted);

        if (!this.displayImported && !this.displayDeleted && !this.displayArchived) {
            this.displayImported = true;
            this.displayArchived = true;
        }
        this.updateStatusFilters();
    }

    private updateStatusFilters() {
        this.deleteAllStatusFilters();
        const filter = this.buildFilterForDisplayProperty();
        this.filters.addFilter(filter, 0);
        this.state.setTagFilters(this.filters);
    }

    private deleteAllStatusFilters() {
        for (const status of ["Imported", "Archived", "Deleted"]) {
            const query = FilterQueryBuilder.status(status as FileStatus);
            this.filters.removeSubfilter(query);
            this.filters.removeFilter({ Query: query });
        }
        this.state.setTagFilters(this.filters);
    }

    private buildFilterForDisplayProperty(): FilterExpression {
        const filters = [];
        if (this.displayImported) {
            filters.push(FilterQueryBuilder.status("Imported"));
        }
        if (this.displayArchived) {
            filters.push(FilterQueryBuilder.status("Archived"));
        }
        if (this.displayDeleted) {
            filters.push(FilterQueryBuilder.status("Deleted"));
        }
        return { OrExpression: filters };
    }
}
