import {
  Component,
  Input,
  OnChanges,
  OnInit,
  SimpleChanges,
  ViewChild
} from '@angular/core';
import {FormControl} from "@angular/forms";
import {File} from "../../models/File";
import {Tag} from "../../models/Tag";
import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {Observable} from "rxjs";
import {map, startWith} from "rxjs/operators";
import {TagService} from "../../services/tag/tag.service";

@Component({
  selector: 'app-file-edit',
  templateUrl: './file-edit.component.html',
  styleUrls: ['./file-edit.component.scss']
})
export class FileEditComponent implements OnInit, OnChanges {

  @Input() files: File[] = [];
  public tags: Tag[] = [];

  private allTags: Tag[] = [];
  private fileTags: {[key: number]: Tag[]} = {};

  public suggestionTags: Observable<string[]>;
  public tagInputForm = new FormControl("");
  public editMode: string = "Toggle";

  @ViewChild("tagScroll") tagScroll!: CdkVirtualScrollViewport;

  constructor(
    private tagService: TagService,
  ) {
    this.suggestionTags = this.tagInputForm.valueChanges.pipe(startWith(null),
      map(
        (tag: string | null) => tag ? this.filterSuggestionTag(
          tag) : this.allTags.slice(0, 20).map(t => t.getNormalizedOutput())));
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

  public async editTagByAutocomplete($event: MatAutocompleteSelectedEvent) {
    const tag = $event.option.value.trim();
    await this.editTag(tag);
  }

  public async editTag(tag: string): Promise<void> {
    if (tag.length > 0) {
      let tagInstance = this.allTags.find(t => t.getNormalizedOutput() === tag);

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
      this.tagInputForm.setValue("");
    }
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
      this.fileTags[file.id] = await this.tagService.changeFileTags(file.id, addedTags, removedTags);
    }
    this.mapFileTagsToTagList();
    const index = this.tags.indexOf(tag);
    index >= 0 && this.tagScroll.scrollToIndex(index);
  }

  async addTag(tag: Tag) {
    for (const file of this.files) {
      if (this.fileTags[file.id].findIndex(t => t.id === tag.id) < 0) {
        this.fileTags[file.id] = await this.tagService.changeFileTags(file.id,
          [tag.id], []);
      }
    }
    if (this.tags.findIndex(t => t.getNormalizedOutput() === tag.getNormalizedOutput()) < 0) {
      this.tags.push(tag);
      this.tags = this.tags.sort(
        (a, b) => a.getNormalizedOutput().localeCompare(b.getNormalizedOutput()));
      this.tags = [...this.tags]; // angular pls detect it wtf
    }
    const index = this.tags.indexOf(tag);
    index >= 0 && this.tagScroll.scrollToIndex(index);
  }

  public async removeTag(tag: Tag) {
    for (const file of this.files) {
      if (this.fileTags[file.id].findIndex(t => t.id === tag.id) >= 0) {
        this.fileTags[file.id] = await this.tagService.changeFileTags(file.id,
          [], [tag.id]);
      }
    }
    const index = this.tags.indexOf(tag);
    if (index >= 0) {
      this.tags.splice(index, 1);
      this.tags = [...this.tags]; // so angular detects the change
    }
  }

  private filterSuggestionTag(tag: string) {
    const allTags = this.allTags.map(t => t.getNormalizedOutput());
    return allTags.filter(
        t => t.includes(tag))
      .slice(0, 20);
  }

  private async loadFileTags() {
    for (const file of this.files) {
      this.fileTags[file.id] = await this.tagService.getTagsForFiles([file.hash]);
    }
    this.mapFileTagsToTagList();
  }

  private mapFileTagsToTagList() {
    let tags: Tag[] = [];
    for (const file of this.files) {
      const fileTags = this.fileTags[file.id];
      tags.push(...fileTags.filter(t => tags.findIndex(tag => tag.id === t.id) < 0));
    }
    this.tags = tags.sort((a, b) => a.getNormalizedOutput().localeCompare(b.getNormalizedOutput()));
  }

  public async handleTagInputKeydown($event: KeyboardEvent) {
    if ($event.key === "Enter") {
      await this.editTag(this.tagInputForm.value);
    }
  }
}
