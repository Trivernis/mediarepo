import {Component, EventEmitter, Input, OnChanges, Output, SimpleChanges} from "@angular/core";
import {Observable} from "rxjs";
import {FormControl} from "@angular/forms";
import {Tag} from "../../../../../api/models/Tag";
import {FilterExpression} from "../../../../../api/api-types/files";
import {debounceTime, map, startWith} from "rxjs/operators";
import {compareSearchResults} from "../../../../utils/compare-utils";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {FilterQueryBuilder} from "../../../../../api/models/FilterQueryBuilder";

@Component({
    selector: "app-filter-input",
    templateUrl: "./filter-input.component.html",
    styleUrls: ["./filter-input.component.scss"]
})
export class FilterInputComponent implements OnChanges {

    @Input() availableTags: Tag[] = [];
    @Output() filterAdded = new EventEmitter<FilterExpression>();

    public autosuggestFilters: Observable<string[]>;
    public formControl = new FormControl();

    private propertyQueriesWithValues: { [key: string]: (string | undefined)[] } = {
        ".status": ["imported", "archived", "deleted"],
        ".fileSize": [undefined],
        ".importedTime": [undefined],
        ".createdTime": [undefined],
        ".changedTime": [undefined],
        ".contentDescriptor": [undefined],
        ".fileId": [undefined],
        ".tagCount": [undefined]
    };
    private comparators = [
        ">",
        "<",
        "="
    ];
    private tagsForAutocomplete: string[] = [];

    constructor() {
        this.autosuggestFilters = this.formControl.valueChanges.pipe(
            startWith(null),
            debounceTime(250),
            map((value) => value ? this.filterAutosuggestFilters(value) : this.tagsForAutocomplete.slice(0, 20))
        );
        this.tagsForAutocomplete = this.availableTags.map(
            t => t.getNormalizedOutput());
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (changes["availableTags"]) {
            this.tagsForAutocomplete = this.availableTags.map(
                t => t.getNormalizedOutput());
        }
    }

    public addFilterByInput(): void {
        const filter = FilterQueryBuilder.buildFilterFromString(this.formControl.value);

        if ("Tag" in filter) {
            const tagFilter = filter["Tag"];

            if (this.tagsForAutocomplete.includes(tagFilter.tag)) {
                this.filterAdded.emit({ Query: filter });
                this.clearFilterInput();
            } else {
                this.formControl.setErrors(["invalid tag"]);
            }
        } else {
            this.filterAdded.emit({ Query: filter });
            this.clearFilterInput();
        }
    }

    public addFilterByAutocomplete(_event: MatAutocompleteSelectedEvent): void {
    }

    private filterAutosuggestFilters(filterValue: string): string[] {
        const trimmedValue = filterValue.trim();
        let isNegation = trimmedValue.startsWith("-");
        const cleanValue = trimmedValue.replace(/^-/, "");
        const autosuggestTags = this.tagsForAutocomplete.filter(t => t.includes(cleanValue)).map(t => isNegation ? "-" + t : t);
        let propertyQuerySuggestions: string[] = [];
        console.error("NEW STATE");

        if (trimmedValue.startsWith(".")) {
            propertyQuerySuggestions = this.buildPropertyQuerySuggestions(trimmedValue);
        }

        return [...autosuggestTags, ...propertyQuerySuggestions].sort((r, l) => compareSearchResults(
            cleanValue,
            r,
            l
        )).slice(0, 50);
    }

    private clearFilterInput() {
        this.formControl.setValue("");
    }

    private buildPropertyQuerySuggestions(trimmedValue: string): string[] {
        const parts = trimmedValue.split(/ |==|=|<|>/g).filter(p => p.length > 0);
        console.log(parts);
        const validProperties = Object.keys(this.propertyQueriesWithValues).filter(q => q.toLowerCase().startsWith(parts[0].trim().toLowerCase()));
        let validComparators = this.comparators.filter(c => trimmedValue.includes(c));

        if (validComparators.length === 0) {
            validComparators = this.comparators;
        }

        let value = "";
        if (parts.length > 1 && !this.comparators.includes(parts[1].trim())) {
            value = parts[1].trim();
        } else if (parts.length > 2) {
            value = parts[2].trim();
        }
        console.log("properties", validProperties, "comparators", validComparators, "value", value);

        if (validComparators.length == 1) {
            return validProperties.map(p => validComparators.map(c => this.propertyQueriesWithValues[p].map(v => `${p} ${c} ${v ?? value}`.trim())).flat()).flat();
        } else {
            return validProperties.map(p => validComparators.map(c => `${p} ${c} ${value}`.trim())).flat();
        }
    }
}
