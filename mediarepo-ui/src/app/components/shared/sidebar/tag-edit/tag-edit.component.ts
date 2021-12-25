import {
    Component,
    Input,
    OnChanges,
    OnInit,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {File} from "../../../../models/File";
import {Tag} from "../../../../models/Tag";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";
import {TagService} from "../../../../services/tag/tag.service";
import {delay} from "rxjs/operators";

@Component({
    selector: "app-tag-edit",
    templateUrl: "./tag-edit.component.html",
    styleUrls: ["./tag-edit.component.scss"]
})
export class TagEditComponent implements OnInit, OnChanges {

    @Input() files: File[] = [];
    public tags: Tag[] = [];

    public allTags: Tag[] = [];
    public editMode: string = "Toggle";
    @ViewChild("tagScroll") tagScroll!: CdkVirtualScrollViewport;
    private fileTags: { [key: number]: Tag[] } = {};

    public loading = false;

    constructor(
        private tagService: TagService,
    ) {
    }

    async ngOnInit() {
        this.tagService.tags.subscribe(tags => this.allTags = tags);
        await this.tagService.loadTags();
        await this.loadFileTags();
    }

    async ngOnChanges(changes: SimpleChanges) {
        if (changes["files"]) {
            await this.loadFileTags()
        }
    }

    public async editTag(tag: string): Promise<void> {
        this.loading = true;
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
        this.loading = false;
    }

    async toggleTag(tag: Tag) {
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
                addedTags, removedTags);
        }
        this.mapFileTagsToTagList();
        const index = this.tags.indexOf(tag);
        index >= 0 && this.tagScroll.scrollToIndex(index);
    }

    async addTag(tag: Tag) {
        for (const file of this.files) {
            if ((this.fileTags[file.id] ?? []).findIndex(t => t.id === tag.id) < 0) {
                this.fileTags[file.id] = await this.tagService.changeFileTags(
                    file.id,
                    [tag.id], []);
            }
        }
        this.mapFileTagsToTagList();
        const index = this.tags.indexOf(tag);
        index >= 0 && this.tagScroll.scrollToIndex(index);
    }

    public async removeTag(tag: Tag) {
        this.loading = true;
        for (const file of this.files) {
            if (this.fileTags[file.id].findIndex(t => t.id === tag.id) >= 0) {
                this.fileTags[file.id] = await this.tagService.changeFileTags(
                    file.id,
                    [], [tag.id]);
            }
        }
        this.mapFileTagsToTagList();
        this.loading = false;
    }

    private async loadFileTags() {
        this.loading = true;
        const promises = [];
        const loadFn = async (file: File) => {
            this.fileTags[file.id] = await this.tagService.getTagsForFiles(
                [file.hash]);
        }
        for (const file of this.files) {
            promises.push(loadFn(file));
        }

        await Promise.all(promises);
        this.mapFileTagsToTagList();
        this.loading = false;
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
}
