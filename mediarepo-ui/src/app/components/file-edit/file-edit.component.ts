import {Component, Input, OnInit, ViewChild} from '@angular/core';
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
export class FileEditComponent implements OnInit {

  @Input() files: File[] = [];
  @Input() tags: Tag[] = [];

  private allTags: Tag[] = [];

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
  }

  public async editTagByAutocomplete($event: MatAutocompleteSelectedEvent) {
    const tag = $event.option.value.trim();
    await this.editTag(tag);
  }

  private async editTag(tag: string): Promise<void> {
    if (tag.length > 0) {
      let tagInstance = this.allTags.find(t => t.getNormalizedOutput() === tag);

      if (!tagInstance) {
        // TODO: Create tag
        tagInstance = new Tag(0, "", undefined);
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

  }

  async addTag(tag: Tag) {
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
}
