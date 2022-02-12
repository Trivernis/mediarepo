import {ChangeDetectionStrategy, Component, EventEmitter, Input, OnChanges, Output, SimpleChanges} from "@angular/core";
import {Observable} from "rxjs";
import {FormControl} from "@angular/forms";
import {Tag} from "../../../../../api/models/Tag";
import {FilterExpression, FilterQuery} from "../../../../../api/api-types/files";
import {debounceTime, map, startWith} from "rxjs/operators";
import {compareSearchResults} from "../../../../utils/compare-utils";
import {FilterQueryBuilder} from "../../../../../api/models/FilterQueryBuilder";
import {normalizeTag} from "../../../../utils/tag-utils";

type AutocompleteEntry = {
    value: string,
    display: string,
};

@Component({
    selector: "app-filter-input",
    templateUrl: "./filter-input.component.html",
    styleUrls: ["./filter-input.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FilterInputComponent implements OnChanges {

    @Input() value: string | undefined;
    @Input() availableTags: Tag[] = [];
    @Output() filterAdded = new EventEmitter<FilterExpression>();

    public autosuggestFilters: Observable<AutocompleteEntry[]>;
    public formControl = new FormControl();

    public skipEnterOnce = false;

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
            map((value) => value ? this.filterAutosuggestFilters(value) : this.tagsForAutocomplete.slice(
                0,
                20
            ).map(t => {
                return { value: t, display: this.buildAutocompleteValue(t) };
            }))
        );
        this.tagsForAutocomplete = this.availableTags.map(
            t => t.getNormalizedOutput());
        if (this.value) {
            this.formControl.setValue(this.value);
        }
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (changes["availableTags"]) {
            this.tagsForAutocomplete = this.availableTags.map(
                t => t.getNormalizedOutput());
        }
        if (changes["value"] && this.value) {
            this.formControl.setValue(this.value);
        }
    }

    public addExpressionByInput(): void {
        if (this.skipEnterOnce) {
            this.skipEnterOnce = false; // workaround to be able to listen to enter (because change is unrelieable) while still allowing enter in autocomplete
            return;
        }
        const expressions = FilterQueryBuilder.buildFilterExpressionsFromString(this.formControl.value);

        let valid: boolean;

        if (expressions && "OrExpression" in expressions) {
            valid = this.validateFilters(expressions.OrExpression);
        } else if (expressions) {
            valid = this.validateFilters([expressions.Query]);
        } else {
            valid = false;
        }

        if (valid) {
            this.filterAdded.emit(expressions);
            this.clearFilterInput();
        } else {
            this.formControl.setErrors(["invalid filters"]);
        }
    }

    public buildAutocompleteValue(value: string): string {
        if (this.formControl.value) {
            const queryParts = this.formControl.value.split(/\s+or\s+/gi);

            if (queryParts.length > 1) {
                value = queryParts.slice(0, queryParts.length - 1).join(" OR ") + " OR " + value;
            }
        }

        return value;
    }

    private validateFilters(filters: FilterQuery[]): boolean {
        for (const filter of filters) {
            if ("Tag" in filter && !filter.Tag.tag.endsWith("*") && !this.tagsForAutocomplete.includes(filter.Tag.tag)) {
                console.debug("tags don't include", filter);
                return false;
            }
        }
        return true;
    }

    private filterAutosuggestFilters(filterValue: string): AutocompleteEntry[] {
        const queryParts = filterValue.toLowerCase().split(/\s+or\s+/gi);
        const latestQuery = queryParts[queryParts.length - 1];
        const trimmedValue = latestQuery.trim();
        let isNegation = trimmedValue.startsWith("-");
        const cleanValue = normalizeTag(trimmedValue.replace(/^-/, ""));
        const autosuggestTags = this.tagsForAutocomplete.filter(t => t.includes(cleanValue)).map(t => isNegation ? "-" + t : t);
        let propertyQuerySuggestions: string[] = [];

        if (trimmedValue.startsWith(".")) {
            propertyQuerySuggestions = this.buildPropertyQuerySuggestions(trimmedValue);
        }

        return [...autosuggestTags, ...propertyQuerySuggestions].sort((r, l) => compareSearchResults(
            cleanValue,
            r,
            l
        )).slice(0, 50).map(e => {
            return {
                display: e,
                value: this.buildAutocompleteValue(e)
            };
        });
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

        if (validComparators.length == 1) {
            return validProperties.map(p => validComparators.filter(c => this.filterComparatorsForProperty(
                c,
                p
            )).map(c => this.propertyQueriesWithValues[p].map(v => `${p} ${c} ${v ?? value}`.trim())).flat()).flat();
        } else {
            return validProperties.map(p => validComparators.filter((c) => this.filterComparatorsForProperty(c, p)).map(
                c => `${p} ${c} ${value}`.trim())).flat();
        }
    }

    private filterComparatorsForProperty(comparator: string, property: string): boolean {
        console.log(comparator, property);
        switch (property) {
            case ".status":
            case ".fileId":
            case ".contentDescriptor":
                return comparator === "=";
            default:
                return true;
        }
    }
}
