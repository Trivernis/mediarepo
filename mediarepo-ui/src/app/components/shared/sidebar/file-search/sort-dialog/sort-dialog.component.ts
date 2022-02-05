import {ChangeDetectionStrategy, ChangeDetectorRef, Component, Inject, OnInit} from "@angular/core";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {SortKey} from "../../../../../../api/models/SortKey";
import {CdkDragDrop, moveItemInArray} from "@angular/cdk/drag-drop";
import {Namespace} from "../../../../../../api/models/Namespace";
import {TagService} from "../../../../../services/tag/tag.service";
import {compareSearchResults} from "../../../../../utils/compare-utils";
import {SortingPreset} from "../../../../../../api/models/SortingPreset";
import {PresetService} from "../../../../../services/preset/preset.service";
import {LoggingService} from "../../../../../services/logging/logging.service";

@Component({
    selector: "app-sort-dialog",
    templateUrl: "./sort-dialog.component.html",
    styleUrls: ["./sort-dialog.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SortDialogComponent implements OnInit {

    public sortingPreset: SortingPreset = SortingPreset.fromValues(-1, []);
    public availablePresets: SortingPreset[] = [];
    public suggestedNamespaces: Namespace[] = [];
    public emptyPreset = SortingPreset.fromValues(-1, []);

    public previousId: number = -1;
    private namespaces: Namespace[] = [];

    constructor(
        public logger: LoggingService,
        public tagService: TagService,
        public presetService: PresetService,
        public changeDetector: ChangeDetectorRef,
        public dialogRef: MatDialogRef<SortDialogComponent>,
        @Inject(
            MAT_DIALOG_DATA) data: any
    ) {
        this.sortingPreset = data.sortingPreset;
        this.previousId = this.sortingPreset.id;
        console.debug(this.sortingPreset);
        tagService.namespaces.subscribe(
            namespaces => this.namespaces = namespaces);
    }

    public async ngOnInit() {
        this.availablePresets = await this.presetService.getAllSortingPresets();
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

    public async savePreset() {
        await this.deletePreset();
        await this.saveNewPreset();
    }

    public async saveNewPreset() {
        let newPreset = await this.logger.try(() => this.presetService.addSortingPreset(this.sortingPreset.sortKeys));
        if (newPreset) {
            this.sortingPreset.setData(newPreset.rawData);
            this.previousId = this.sortingPreset.id;
            this.availablePresets.push(new SortingPreset(JSON.parse(JSON.stringify(newPreset.rawData))));
            this.changeDetector.detectChanges();
        }
    }

    public async deletePreset() {
        if (this.previousId >= 0) {
            const index = this.availablePresets.findIndex(p => p.id == this.previousId);
            if (index >= 0) {
                this.availablePresets.splice(index, 1);
                this.changeDetector.detectChanges();
            }
            try {
                await this.presetService.deleteSortingPreset(this.previousId);
            } catch (err: any) {
                this.logger.warn(`Could not delete previous preset: ${err.message}`);
            }
        }
    }

    public selectPreset(presetId: number): void {
        const preset = this.availablePresets.find(p => p.id == presetId) ?? this.emptyPreset;

        if (preset) {
            this.sortingPreset.setData(JSON.parse(JSON.stringify(preset.rawData)));
            this.previousId = preset.id;
        }
    }
}
