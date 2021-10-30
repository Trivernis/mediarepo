import {
  AfterViewChecked,
  Component,
  ElementRef,
  ViewChild
} from '@angular/core';
import {TagService} from "../../services/tag/tag.service";
import {FileService} from "../../services/file/file.service";
import {FormControl} from "@angular/forms";
import {COMMA} from "@angular/cdk/keycodes";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {map, startWith} from "rxjs/operators";
import {Observable} from "rxjs";
import {TagQuery} from "../../models/TagQuery";

@Component({
  selector: 'app-file-search',
  templateUrl: './file-search.component.html',
  styleUrls: ['./file-search.component.scss']
})
export class FileSearchComponent implements AfterViewChecked {
  public ngAfterViewChecked(): void {
    this.inputList.nativeElement.scrollLeft = this.inputList.nativeElement.scrollWidth;
  }

  public searchInputSeparators = [COMMA];
  public formControl = new FormControl();
  public searchTags: TagQuery[] = [];
  public suggestionTags: Observable<string[]>;
  private allTags: string[] = [];

  @ViewChild("tagInput") tagInput!: ElementRef<HTMLInputElement>;
  @ViewChild("tagInputList") inputList!: ElementRef;

  constructor(private tagService: TagService, private fileService: FileService) {
    this.tagService.tags.subscribe(
      (tag) => this.allTags = tag.map(t => t.getNormalizedOutput()));

    this.suggestionTags = this.formControl.valueChanges.pipe(startWith(null),
      map(
        (tag: string | null) => tag ? this.allTags.filter(
            (t: string) => t.includes(tag.replace(/^-/g, '')))
          .map((t) => tag.startsWith("-") ? "-" + t : t)
          .slice(0, 20) : this.allTags.slice(0, 20)));
  }

  public async searchForFiles() {
    await this.fileService.findFiles(this.searchTags);
  }

  public addSearchTag(tag: string) {
    if (tag.startsWith("-")) {
      this.searchTags.push(new TagQuery(tag.replace(/^-/g, ''), true));
    } else {
      this.searchTags.push(new TagQuery(tag, false));
    }
  }

  async removeAllSearchTags() {
    this.searchTags = [];
    await  this.searchForFiles();
  }

  async removeSearchTag(tag: TagQuery) {
    const index = this.searchTags.indexOf(tag);
    if (index >= 0) {
      this.searchTags.splice(index, 1);
    }
    await this.searchForFiles();
  }

  async addSearchTagByInput(event: KeyboardEvent) {
    if (event.key === "Enter") {
      const tag = (this.formControl.value as string ?? "").trim();
      if (tag.length > 0 && this.allTags.includes(tag.replace(/-/g, ''))) {
        this.addSearchTag(tag);
        this.formControl.setValue(null);
        await this.searchForFiles();
      }
    }
  }

  async addSearchTagByAutocomplete(event: MatAutocompleteSelectedEvent) {
    const tag = event.option.viewValue;
    this.addSearchTag(tag);
    this.formControl.setValue(null);
    this.tagInput.nativeElement.value = '';
    await this.searchForFiles();
  }
}
