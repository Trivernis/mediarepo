import {
    Component,
    EventEmitter,
    Input,
    OnChanges,
    OnInit,
    Output,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {Tag} from "../../../../../api/models/Tag";
import {TagService} from "../../../../services/tag/tag.service";
import {File} from "../../../../../api/models/File";
import {
    FileSearchComponent
} from "../../../shared/sidebar/file-search/file-search.component";
import {
    RepositoryService
} from "../../../../services/repository/repository.service";
import {
    TagEditComponent
} from "../../../shared/sidebar/tag-edit/tag-edit.component";
import {TabState} from "../../../../models/TabState";

@Component({
    selector: "app-files-tab-sidebar",
    templateUrl: "./files-tab-sidebar.component.html",
    styleUrls: ["./files-tab-sidebar.component.scss"]
})
export class FilesTabSidebarComponent implements OnInit, OnChanges {

    @Input() state!: TabState;
    @Input() selectedFiles: File[] = [];
    @Output() searchStartEvent = new EventEmitter<void>();
    @Output() searchEndEvent = new EventEmitter<void>();

    @ViewChild("filesearch") fileSearch!: FileSearchComponent;
    @ViewChild("fileedit") fileEdit: TagEditComponent | undefined;

    public tagsOfFiles: Tag[] = [];
    public tags: Tag[] = [];
    public allTags: Tag[] = [];
    public files: File[] = [];
    public tagsOfSelection: Tag[] = [];

    constructor(private repoService: RepositoryService, private tagService: TagService) {
        this.repoService.selectedRepository.subscribe(
            async (repo) => repo && this.fileSearch && await this.fileSearch.searchForFiles());
        this.tagService.tags.subscribe(t => this.allTags = t);
    }

    async ngOnInit() {
        this.state.files.subscribe(async (files) => {
            this.files = files;
            await this.onDisplayedFilesChange();
        });
        if (this.fileSearch) {
            await this.fileSearch.searchForFiles();
        }
        if (this.tags.length === 0 && this.selectedFiles.length === 0) {
            this.tags = this.tagsOfFiles;
        }
    }

    public async ngOnChanges(changes: SimpleChanges): Promise<void> {
        if (changes["selectedFiles"]) {
            await this.showFileDetails(this.selectedFiles);
            this.showAllTagsFallback();
        }
    }

    public async onDisplayedFilesChange() {
        await this.loadTagsForDisplayedFiles();
        await this.refreshFileSelection();
    }

    async loadTagsForDisplayedFiles() {
        this.tagsOfFiles = await this.tagService.getTagsForFiles(
            this.files.map(f => f.cd));
        this.showAllTagsFallback();
    }

    async showFileDetails(files: File[]) {
        this.tagsOfSelection = await this.tagService.getTagsForFiles(
            files.map(f => f.cd));
        this.tagsOfSelection = this.tagsOfSelection.sort(
            (a, b) => a.getNormalizedOutput()
                .localeCompare(b.getNormalizedOutput()));
        this.tags = this.tagsOfSelection;
    }

    private async refreshFileSelection() {
        const filteredSelection = this.selectedFiles.filter(
            file => this.files.findIndex(f => f.id === file.id) >= 0);
        if (filteredSelection.length === 0) {
            this.tags = [];
            this.showAllTagsFallback();
        } else if (filteredSelection.length < this.selectedFiles.length) {
            this.selectedFiles = filteredSelection;
            await this.showFileDetails(this.selectedFiles);
        }
    }

    private showAllTagsFallback() {
        if (this.tags.length === 0 && this.selectedFiles.length === 0) {
            this.tags = this.tagsOfFiles.sort(
                (a, b) => a.getNormalizedOutput()
                    .localeCompare(b.getNormalizedOutput()));
        }
    }
}
