import {Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {TagService} from "../../services/tag/tag.service";
import {FileService} from "../../services/file/file.service";
import {FormControl} from "@angular/forms";
import {COMMA, ENTER} from "@angular/cdk/keycodes";
import {MatChipInputEvent} from "@angular/material/chips";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {map, startWith} from "rxjs/operators";
import {Observable} from "rxjs";

@Component({
  selector: 'app-file-search',
  templateUrl: './file-search.component.html',
  styleUrls: ['./file-search.component.scss']
})
export class FileSearchComponent {

  public searchInputSeparators = [ENTER, COMMA];
  public formControl = new FormControl();
  public searchTags: string[] = [];
  public suggestionTags: Observable<string[]>;
  private allTags: string[] = [];

  @ViewChild('tagInput') tagInput!: ElementRef<HTMLInputElement>;

  constructor(private tagService: TagService, private fileService: FileService) {
    this.tagService.tags.subscribe(
      (tag) => this.allTags = tag.map(t => t.getNormalizedOutput()));

    this.suggestionTags = this.formControl.valueChanges.pipe(startWith(null), map(
      (tag: string | null) => tag ? this.allTags.filter(
        (t: string) => t.includes(tag)).slice(0, 20) : this.allTags.slice(0, 20)));
  }

  public async searchForFiles() {
    await this.fileService.findFiles(this.searchTags);
  }

  public addSearchTag(tag: string) {
    this.searchTags.push(tag);
  }

  async removeSearchTag(tag: string) {
    const index = this.searchTags.indexOf(tag);
    if (index >= 0) {
      this.searchTags.splice(index, 1);
    }
    await this.searchForFiles();
  }

  async addSearchTagByChip(event: MatChipInputEvent) {
    const tag = event.value.trim();
    if (tag.length > 0 && this.allTags.includes(tag)) {
      this.searchTags.push(tag);
      event.chipInput?.clear();
      this.formControl.setValue(null);
      await this.searchForFiles();    }
  }

  async addSearchTagByAutocomplete(event: MatAutocompleteSelectedEvent) {
    const tag = event.option.viewValue;
    this.searchTags.push(tag);
    this.formControl.setValue(null);
    this.tagInput.nativeElement.value = '';
    await this.searchForFiles();  }
}
