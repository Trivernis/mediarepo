import {
    Component,
    ElementRef,
    EventEmitter,
    Input,
    OnChanges,
    Output,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {Tag} from "../../../../models/Tag";
import {FormControl} from "@angular/forms";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {Observable} from "rxjs";
import {debounceTime, map, startWith} from "rxjs/operators";

@Component({
    selector: "app-tag-input",
    templateUrl: "./tag-input.component.html",
    styleUrls: ["./tag-input.component.scss"]
})
export class TagInputComponent implements OnChanges {

    @Input() availableTags: Tag[] = [];
    @Input() allowNegation: boolean = false;
    @Input() allowInvalid: boolean = false;
    @Input() allowWildcards: boolean = false;
    @Output() tagAdded = new EventEmitter<string>();

    @ViewChild("tagInput") tagInput!: ElementRef<HTMLInputElement>;
    public formControl = new FormControl();
    public autosuggestTags: Observable<string[]>;
    private tagsForAutocomplete: string[] = [];

    constructor() {
        this.tagsForAutocomplete = this.availableTags.map(
            t => t.getNormalizedOutput());
        this.autosuggestTags = this.formControl.valueChanges.pipe(
            startWith(null),
            debounceTime(250),
            map((tag: string | null) => tag ? this.filterSuggestionTag(
                tag) : this.tagsForAutocomplete.slice(0, 20)));
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (changes["availableTags"]) {
            this.tagsForAutocomplete = this.availableTags.map(
                t => t.getNormalizedOutput());
        }
    }

    public addTagByInput(event: KeyboardEvent): void {
        if (event.key === "Enter") {
            this.addTag(this.formControl.value);
        }
    }

    public addTagByAutocomplete(event: MatAutocompleteSelectedEvent): void {
        this.addTag(event.option.value);
    }

    private addTag(value: string) {
        const tag = this.normalizeTag(value);
        if (tag.length > 0 && (this.allowInvalid || this.checkTagValid(tag))) {
            this.tagAdded.emit(tag);
            this.formControl.setValue("");
            this.tagInput.nativeElement.value = "";
        }
    }

    private filterSuggestionTag(tag: string) {
        let normalizedTag = this.normalizeTag(tag);
        const negated = normalizedTag.startsWith("-") && this.allowNegation;
        normalizedTag = this.allowNegation ? normalizedTag.replace(/^-/,
            "") : normalizedTag;
        const containsWildcard = normalizedTag.endsWith("*");
        normalizedTag = this.allowWildcards ? normalizedTag.replace(/\*\s*$/,
            "") : normalizedTag;

        const autocompleteTags = this.tagsForAutocomplete.filter(
            t => t.includes(normalizedTag))
            .map(t => negated ? "-" + t : t)
            .sort((l, r) => this.compareSuggestionTags(normalizedTag, l, r))
            .slice(0, 50);

        if (containsWildcard) {
            autocompleteTags.unshift(this.normalizeTag(tag));
        }

        return autocompleteTags;
    }

    private checkTagValid(tag: string): boolean {
        if (this.allowNegation) {
            tag = tag.replace(/^-/, "");
        }
        if (tag.endsWith("*")) {
            if (this.allowWildcards) {
                return this.tagsForAutocomplete.findIndex(
                    t => t.startsWith(tag.trim().replace(/\*\s*$/, ""))) >= 0;
            } else {
                return false;
            }
        }
        return this.tagsForAutocomplete.includes(tag);
    }

    /**
     * Normalizes the tag by removing whitespaces
     * @param {string} tag
     * @returns {string}
     * @private
     */
    private normalizeTag(tag: string): string {
        let normalizedTag = tag.trim();
        let parts = normalizedTag.split(":");

        if (parts.length > 1) {
            const namespace = parts.shift()!.trim();
            const name = parts.join(":").trim();
            return namespace + ":" + name;
        } else {
            return normalizedTag;
        }
    }

    private compareSuggestionTags(query: string, l: string, r: string): number {
        if (l.startsWith(query) && !r.startsWith(query)) {
            return -1;
        } else if (!l.startsWith(query) && r.startsWith(query)) {
            return 1;
        } else if (l.length < r.length) {
            return -1;
        } else if (l.length > r.length) {
            return 1;
        } else {
            return l.localeCompare(r)
        }
    }
}
