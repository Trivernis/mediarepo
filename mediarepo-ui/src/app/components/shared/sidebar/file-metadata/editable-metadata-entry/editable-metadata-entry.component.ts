import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {FormControl} from "@angular/forms";

@Component({
    selector: "app-editable-metadata-entry",
    templateUrl: "./editable-metadata-entry.component.html",
    styleUrls: ["./editable-metadata-entry.component.scss"]
})
export class EditableMetadataEntryComponent implements OnInit{

    @Input() attributeName!: string;
    @Input() value!: string;
    @Input() mode: "read" | "write" = "read";
    @Output() valueChangeEvent = new EventEmitter<string>();

    public formControl = new FormControl();

    constructor() {
    }

    public ngOnInit(): void {
        this.formControl.setValue(this.value);
    }

    public onSave(): void {
        this.valueChangeEvent.emit(this.formControl.value);
        this.mode = "read";
    }
}
