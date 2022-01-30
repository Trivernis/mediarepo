import {
    AfterViewInit,
    ChangeDetectionStrategy,
    Component,
    EventEmitter,
    Input,
    OnChanges,
    Output,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {Tag} from "../../../../../api/models/Tag";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";
import {TagService} from "../../../../services/tag/tag.service";
import {LoggingService} from "../../../../services/logging/logging.service";
import {BusyIndicatorComponent} from "../../app-common/busy-indicator/busy-indicator.component";

@Component({
    selector: "app-tag-edit",
    templateUrl: "./tag-edit.component.html",
    styleUrls: ["./tag-edit.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class TagEditComponent implements AfterViewInit, OnChanges {

    @Input() files: File[] = [];
    @Output() tagEditEvent = new EventEmitter<TagEditComponent>();

    @ViewChild("tagScroll") tagScroll!: CdkVirtualScrollViewport;
    @ViewChild(BusyIndicatorComponent) busyIndicator!: BusyIndicatorComponent;

    public tags: Tag[] = [];
    public allTags: Tag[] = [];
    public editMode: string = "Toggle";
    private fileTags: { [key: number]: Tag[] } = {};

    constructor(
        private logger: LoggingService,
        private tagService: TagService,
    ) {
    }

    async ngAfterViewInit() {
        this.tagService.tags.subscribe(tags => this.allTags = tags);
        await this.tagService.loadTags();
        await this.tagService.loadNamespaces();
        await this.loadFileTags();
    }

    async ngOnChanges(changes: SimpleChanges) {
        if (changes["files"]) {
            await this.loadFileTags();
        }
    }

    public async editTag(tag: string): Promise<void> {
        if (tag.length > 0) {
            let tagInstance = this.allTags.find(
                t => t.getNormalizedOutput() === tag);

            if (!tagInstance) {
                tagInstance = (await this.tagService.createTags([tag]))[0];
                this.allTags.push(tagInstance);
            }
            switch (this.editMode) {
                case "Toggle":
                    await this.toggleTag(tagInstance);
                    break;
                case "Add":
                    await this.addTag(tagInstance);
                    break;
                case "Remove":
                    await this.removeTag(tagInstance);
                    break;
            }
        }
    }

    async toggleTag(tag: Tag) {
        await this.wrapAsyncOperation(async () => {
            for (const file of this.files) {
                const fileTags = this.fileTags[file.id];
                let addedTags = [];
                let removedTags = [];
                if (fileTags.findIndex(i => i.id === tag.id) < 0) {
                    addedTags.push(tag.id);
                } else {
                    removedTags.push(tag.id);
                }
                this.fileTags[file.id] = await this.tagService.changeFileTags(
                    file.id,
                    addedTags, removedTags
                );
                if (addedTags.length > 0) {
                    await this.tagService.loadTags();
                    await this.tagService.loadNamespaces();
                }
            }
            this.mapFileTagsToTagList();
            const index = this.tags.indexOf(tag);
            if (index >= 0) {
                this.tagScroll.scrollToIndex(index);
            }
        });
        this.tagEditEvent.emit(this);
    }

    async addTag(tag: Tag) {
        await this.wrapAsyncOperation(async () => {
            for (const file of this.files) {
                if ((this.fileTags[file.id] ?? []).findIndex(t => t.id === tag.id) < 0) {
                    this.fileTags[file.id] = await this.tagService.changeFileTags(
                        file.id,
                        [tag.id], []
                    );
                }
            }
            this.mapFileTagsToTagList();
            const index = this.tags.indexOf(tag);
            if (index >= 0) {
                this.tagScroll.scrollToIndex(index);
            }
            await this.tagService.loadTags();
            await this.tagService.loadNamespaces();
        });
        this.tagEditEvent.emit(this);
    }

    public async removeTag(tag: Tag) {
        await this.wrapAsyncOperation(async () => {
            for (const file of this.files) {
                if (this.fileTags[file.id].findIndex(t => t.id === tag.id) >= 0) {
                    this.fileTags[file.id] = await this.tagService.changeFileTags(
                        file.id,
                        [], [tag.id]
                    );
                }
            }
            this.mapFileTagsToTagList();
        });
        this.tagEditEvent.emit(this);
    }

    public trackByTagId(index: number, item: Tag) {
        return item.id;
    }

    private async loadFileTags() {
        await this.wrapAsyncOperation(async () => {
            console.log("loading tags");
            const mappings = await this.tagService.getFileTagMappings(this.files.map(f => f.cd));

            for (const file of this.files) {
                this.fileTags[file.id] = mappings[file.cd];
            }
            this.mapFileTagsToTagList();
        });
    }

    private mapFileTagsToTagList() {
        let tags: Tag[] = [];
        for (const file of this.files) {
            const fileTags = this.fileTags[file.id];
            tags.push(
                ...fileTags.filter(
                    t => tags.findIndex(tag => tag.id === t.id) < 0));
        }
        this.tags = tags.sort(
            (a, b) => a.getNormalizedOutput()
                .localeCompare(b.getNormalizedOutput()));
    }

    private async wrapAsyncOperation<T>(cb: () => Promise<T>): Promise<T | undefined> {
        if (!this.busyIndicator?.wrapAsyncOperation) {
            try {
                return cb();
            } catch (err: any) {
                this.logger.error(err);
                return undefined;
            }
        } else {
            return this.busyIndicator.wrapAsyncOperation(cb);
        }
    }
}
