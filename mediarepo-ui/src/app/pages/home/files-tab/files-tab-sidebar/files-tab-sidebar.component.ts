import {
  Component,
  EventEmitter,
  Input, OnChanges,
  OnInit,
  Output, SimpleChanges,
  ViewChild
} from '@angular/core';
import {Tag} from "../../../../models/Tag";
import {TagService} from "../../../../services/tag/tag.service";
import {FileService} from "../../../../services/file/file.service";
import {File} from "../../../../models/File";
import {MatSelectionListChange} from "@angular/material/list";
import {FileSearchComponent} from "../../../../components/file-search/file-search.component";
import {RepositoryService} from "../../../../services/repository/repository.service";

@Component({
  selector: 'app-files-tab-sidebar',
  templateUrl: './files-tab-sidebar.component.html',
  styleUrls: ['./files-tab-sidebar.component.scss']
})
export class FilesTabSidebarComponent implements OnInit, OnChanges {

  @Input() selectedFiles: File[] = [];
  @Output() searchStartEvent = new EventEmitter<void>();
  @Output() searchEndEvent = new EventEmitter<void>();

  @ViewChild('filesearch') fileSearch!: FileSearchComponent;

  public tagsOfFiles: Tag[] = [];
  public tags: Tag[] = [];
  public files: File[] = [];

  constructor(private repoService: RepositoryService, private tagService: TagService, private fileService: FileService) {
    this.fileService.displayedFiles.subscribe(async files => {
      this.files = files;
      await this.loadTagsForDisplayedFiles();
    });
    this.repoService.selectedRepository.subscribe(async (repo) => repo && this.fileSearch && await this.fileSearch.searchForFiles());
  }

  async ngOnInit() {
    this.fileSearch && await this.fileSearch.searchForFiles();
  }

  public async ngOnChanges(changes: SimpleChanges): Promise<void >{
    if (changes["selectedFiles"]) {
      const tags = await this.tagService.getTagsForFiles(this.selectedFiles.map(f => f.hash));
      this.tags = tags.sort((a, b) => a.getNormalizedOutput().localeCompare(b.getNormalizedOutput()));
    }
  }

  async loadTagsForDisplayedFiles() {
    this.tagsOfFiles = await this.tagService.getTagsForFiles(this.files.map(f => f.hash));
  }

  async addSearchTagFromList(event: MatSelectionListChange) {
    if (event.options.length > 0) {
      const tag = event.options[0].value;
      this.fileSearch.addSearchTag(tag);
      await this.fileSearch.searchForFiles();
    }
    event.source.deselectAll();
  }

  getValidTagsForSearch(): string[] {
    return this.tagsOfFiles.map(t => t.getNormalizedOutput())
  }

  async showFileDetails(files: File[]) {
    this.tags = await this.tagService.getTagsForFiles(files.map(f => f.hash))
    this.tags = this.tags.sort((a, b) => a.getNormalizedOutput().localeCompare(b.getNormalizedOutput()));
  }
}
