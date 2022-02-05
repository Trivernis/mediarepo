import {ChangeDetectionStrategy, Component, Inject} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {SortKey} from "../../../../../../api/models/SortKey";
import {CdkDragDrop, moveItemInArray} from "@angular/cdk/drag-drop";
import {Namespace} from "../../../../../../api/models/Namespace";
import {TagService} from "../../../../../services/tag/tag.service";
import {compareSearchResults} from "../../../../../utils/compare-utils";
import {SortingPreset} from "../../../../../../api/models/SortingPreset";

@Component({
    selector: "app-sort-dialog",
    templateUrl: "./sort-dialog.component.html",
    styleUrls: ["./sort-dialog.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SortDialogComponent {

    public sortingPreset: SortingPreset = SortingPreset.fromValues(-1, []);
    public suggestedNamespaces: Namespace[] = [];

    private previousId: number = -1;
    private namespaces: Namespace[] = [];

    constructor(public tagService: TagService, public dialogRef: MatDialogRef<SortDialogComponent>, @Inject(
        MAT_DIALOG_DATA) data: any) {
        this.sortingPreset = data.sortingPreset;
        console.debug(this.sortingPreset);
        tagService.namespaces.subscribe(
            namespaces => this.namespaces = namespaces);
    }

    addNewSortKey() {
        const sortKey = SortKey.fromValues("FileName", "Ascending", undefined);
        this.handlePresetChange();
        this.sortingPreset.sortKeys.push(sortKey);
    }

    public removeSortKey(sortKey: SortKey): void {
        const index = this.sortingPreset.sortKeys.indexOf(sortKey);
        this.handlePresetChange();
        this.sortingPreset.sortKeys.splice(index, 1);
    }

    public confirmSort(): void {
        this.dialogRef.close(this.sortingPreset);
    }

    public cancelSort(): void {
        this.dialogRef.close();
    }

    public onSortEntryDrop(event: CdkDragDrop<SortKey[]>): void {
        this.handlePresetChange();
        moveItemInArray(this.sortingPreset.sortKeys, event.previousIndex,
            event.currentIndex
        );
    }

    public updateAutocompleteSuggestions(value: string): void {
        this.suggestedNamespaces = this.namespaces.sort(
            (a, b) => compareSearchResults(value, a.name, b.name))
            .slice(0, 50);
    }

    public handlePresetChange() {
        if (this.sortingPreset.id >= 0) {
            this.previousId = this.sortingPreset.id;
            this.sortingPreset.id = -1;
        }
    }
}
