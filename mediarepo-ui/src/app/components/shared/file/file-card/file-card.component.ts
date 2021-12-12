import {
    Component,
    ElementRef,
    EventEmitter,
    Input,
    OnChanges,
    OnInit,
    Output,
    SimpleChanges,
    ViewChild
} from "@angular/core";
import {File} from "../../../../models/File";
import {Selectable} from "../../../../models/Selectable";

@Component({
    selector: "app-file-card",
    templateUrl: "./file-card.component.html",
    styleUrls: ["./file-card.component.scss"]
})
export class FileCardComponent implements OnInit, OnChanges {

    @ViewChild("card") card!: ElementRef;
    @Input() public entry!: Selectable<File>;
    @Output() clickEvent = new EventEmitter<FileCardComponent>();
    @Output() dblClickEvent = new EventEmitter<FileCardComponent>();

    private cachedId: number | undefined;
    private urlSetTimeout: number | undefined;
    public loading = false;

    constructor() {
    }

    async ngOnInit() {
        this.cachedId = this.entry.data.id;
        this.setImageDelayed();
    }

    async ngOnChanges(changes: SimpleChanges) {
        if (changes["file"] && (this.cachedId === undefined || this.entry.data.id !== this.cachedId)) {
            this.cachedId = this.entry.data.id;
            this.setImageDelayed();
        }
    }

    private setImageDelayed() {
        this.loading = true;
        clearTimeout(this.urlSetTimeout);
        this.urlSetTimeout = setTimeout(
            () => this.loading = false, 200);
    }
}
