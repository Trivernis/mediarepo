import {
    AfterViewChecked,
    Component,
    ElementRef,
    EventEmitter,
    Input,
    OnInit,
    Output,
    ViewChild
} from "@angular/core";
import {TagQuery} from "../../../../models/TagQuery";
import {SortKey} from "../../../../models/SortKey";
import {MatDialog} from "@angular/material/dialog";
import {SortDialogComponent} from "./sort-dialog/sort-dialog.component";
import {
    ErrorBrokerService
} from "../../../../services/error-broker/error-broker.service";
import {
    GenericFilter,
    SingleFilterExpression
} from "../../../../models/GenericFilter";
import {FilterDialogComponent} from "./filter-dialog/filter-dialog.component";
import {Tag} from "../../../../../api/models/Tag";
import {clipboard} from "@tauri-apps/api";
import {TabState} from "../../../../models/TabState";


@Component({
    selector: "app-file-search",
    templateUrl: "./file-search.component.html",
    styleUrls: ["./file-search.component.scss"]
})
export class FileSearchComponent implements AfterViewChecked, OnInit {
    public sortExpression: SortKey[] = [];
    public filters: GenericFilter[] = [];

    @Input() availableTags: Tag[] = [];
    @Input() contextTags: Tag[] = [];
    @Input() state!: TabState;

    @Output() searchStartEvent = new EventEmitter<void>();
    @Output() searchEndEvent = new EventEmitter<void>();

    @ViewChild("tagInput") tagInput!: ElementRef<HTMLInputElement>;
    @ViewChild("tagInputList") inputList!: ElementRef;

    public contextMenuTag: Tag | undefined;

    constructor(
        private errorBroker: ErrorBrokerService,
        public dialog: MatDialog
    ) {
    }

    public async ngOnInit() {
        this.state.filters.subscribe(f => this.filters = f);
        this.state.sortKeys.subscribe(s => this.sortExpression = s);
        await this.searchForFiles();
    }

    public ngAfterViewChecked(): void {
        this.inputList.nativeElement.scrollLeft = this.inputList.nativeElement.scrollWidth;
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

    public addSearchTag(tag: string) {
        this.filters.push(new SingleFilterExpression(TagQuery.fromString(tag)));
        tag = tag.replace(/^-/g, "");

        if (this.filters.filter(t => t.partiallyEq(tag)).length > 1) {
            const index = this.filters.findIndex(t => t.partiallyEq(tag));
            this.filters.splice(index, 1);
        }
        this.state.setFilters(this.filters);
    }

    public getValidSearchTags(): Tag[] {
        return this.availableTags.filter(t => this.filters.findIndex(
            f => f.partiallyEq(t.getNormalizedOutput())) < 0);
    }

    public async removeAllSearchTags() {
        this.filters = [];
        this.state.setFilters([]);
    }

    public async removeFilterExpression(expr: GenericFilter) {
        const index = this.filters.indexOf(expr);
        if (index >= 0) {
            this.filters.splice(index, 1);
        }
        this.state.setFilters(this.filters);
    }

    public openSortDialog() {
        const sortEntries = this.sortExpression.map(
            key => JSON.parse(JSON.stringify(key))).map(
            key => new SortKey(key.sortType, key.sortDirection,
                key.namespaceName));
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
        const filterEntries = this.filters.map(f => f.clone());
        const filterDialog = this.dialog.open(FilterDialogComponent, {
            minWidth: "25vw",
            maxHeight: "80vh",
            data: {
                filterEntries,
                availableTags: this.availableTags,
            },
            disableClose: true,
        });
        filterDialog.afterClosed().subscribe(async (filterExpression) => {
            if (filterExpression !== undefined || filterExpression?.length > 0) {
                this.filters = filterExpression;
                this.state.setFilters(this.filters);
            }
        });
    }

    public async copyToClipboard(text: string) {
        await clipboard.writeText(text);
    }
}
