import {AfterViewChecked, Component, ElementRef, EventEmitter, Input, OnInit, Output, ViewChild} from "@angular/core";
import {SortKey} from "../../../../models/SortKey";
import {MatDialog} from "@angular/material/dialog";
import {SortDialogComponent} from "./sort-dialog/sort-dialog.component";
import {ErrorBrokerService} from "../../../../services/error-broker/error-broker.service";
import {FilterDialogComponent} from "./filter-dialog/filter-dialog.component";
import {Tag} from "../../../../../api/models/Tag";
import {clipboard} from "@tauri-apps/api";
import {TabState} from "../../../../models/TabState";
import {FilterQueryBuilder} from "../../../../../api/models/FilterQueryBuilder";
import {SearchFilters} from "../../../../../api/models/SearchFilters";
import {FileStatus, FilterExpression,} from "../../../../../api/api-types/files";
import {filterExpressionToString} from "../../../../utils/filter-utils";
import {MatCheckboxChange} from "@angular/material/checkbox";
import * as deepEqual from "fast-deep-equal";


@Component({
    selector: "app-file-search",
    templateUrl: "./file-search.component.html",
    styleUrls: ["./file-search.component.scss"]
})
export class FileSearchComponent implements AfterViewChecked, OnInit {
    public sortExpression: SortKey[] = [];
    public filters: SearchFilters = new SearchFilters([]);

    @Input() availableTags: Tag[] = [];
    @Input() contextTags: Tag[] = [];
    @Input() state!: TabState;
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

    private needsScroll = false;

    constructor(
        private errorBroker: ErrorBrokerService,
        public dialog: MatDialog
    ) {
        this.assignDisplayedFilters();
    }

    public async ngOnInit() {
        this.state.filters.subscribe(f => {
            this.filters = f;
            this.assignDisplayedFilters();
        });
        this.state.sortKeys.subscribe(s => this.sortExpression = s);
        this.applyStatusFromFilters();
        await this.searchForFiles();
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
        } catch (err) {
            this.errorBroker.showError(err);
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
    }

    public async removeFilterExpression(expr: FilterExpression) {
        this.filters.removeFilter(expr);
        this.state.setTagFilters(this.filters);
        this.needsScroll = true;
    }

    public openSortDialog() {
        const sortEntries = this.sortExpression.map(
            key => JSON.parse(JSON.stringify(key))).map(
            key => new SortKey(key.sortType, key.sortDirection,
                key.namespaceName
            ));
        const openedDialog = this.dialog.open(SortDialogComponent, {
            minWidth: "40vw",
            data: {
                sortEntries,
            },
            disableClose: true,
        });
        openedDialog.afterClosed().subscribe(async (sortExpression) => {
            if (sortExpression) {
                this.sortExpression = sortExpression;
                this.state.setSortKeys(this.sortExpression);
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
