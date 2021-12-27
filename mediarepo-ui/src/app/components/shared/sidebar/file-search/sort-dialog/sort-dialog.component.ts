import {Component, Inject} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {SortKey} from "../../../../../models/SortKey";
import {CdkDragDrop, moveItemInArray} from "@angular/cdk/drag-drop";
import {Namespace} from "../../../../../models/Namespace";
import {TagService} from "../../../../../services/tag/tag.service";
import {FormControl} from "@angular/forms";

@Component({
    selector: "app-sort-dialog",
    templateUrl: "./sort-dialog.component.html",
    styleUrls: ["./sort-dialog.component.scss"]
})
export class SortDialogComponent {

    public sortEntries: SortKey[] = []
    public suggestedNamespaces: Namespace[] = [];

    private namespaces: Namespace[] = [];

    constructor(public tagService: TagService, public dialogRef: MatDialogRef<SortDialogComponent>, @Inject(
        MAT_DIALOG_DATA) data: any) {
        this.sortEntries = data.sortEntries;
        tagService.namespaces.subscribe(
            namespaces => this.namespaces = namespaces);
    }

    addNewSortKey() {
        const sortKey = new SortKey("FileName", "Ascending", undefined);
        this.sortEntries.push(sortKey)
    }

    public removeSortKey(sortKey: SortKey): void {
        const index = this.sortEntries.indexOf(sortKey);
        this.sortEntries.splice(index, 1);
    }

    public confirmSort(): void {
        this.dialogRef.close(this.sortEntries);
    }

    public cancelSort(): void {
        this.dialogRef.close()
    }

    public onSortEntryDrop(event: CdkDragDrop<SortKey[]>): void {
        moveItemInArray(this.sortEntries, event.previousIndex,
            event.currentIndex);
    }

    public updateAutocompleteSuggestions(value: string): void {
        this.suggestedNamespaces = this.namespaces.sort(
            (a, b) => this.compareSuggestionNamespaces(value, a.name, b.name))
            .slice(0, 50)
    }

    private compareSuggestionNamespaces(query: string, l: string, r: string): number {
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
