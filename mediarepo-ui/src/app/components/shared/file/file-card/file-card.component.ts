import {
    ChangeDetectionStrategy,
    ChangeDetectorRef,
    Component,
    EventEmitter,
    Input,
    OnChanges,
    OnDestroy,
    OnInit,
    Output,
    SimpleChanges
} from "@angular/core";
import {File} from "../../../../../api/models/File";
import {Selectable} from "../../../../models/Selectable";
import {SchedulingService} from "../../../../services/scheduling/scheduling.service";

const LOADING_WORK_KEY = "FILE_THUMBNAIL_LOADING";

@Component({
    selector: "app-file-card",
    templateUrl: "./file-card.component.html",
    styleUrls: ["./file-card.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FileCardComponent implements OnInit, OnChanges, OnDestroy {

    @Input() public entry!: Selectable<File>;
    @Output() clickEvent = new EventEmitter<FileCardComponent>();
    @Output() dblClickEvent = new EventEmitter<FileCardComponent>();

    public loading = false;
    private cachedId: number | undefined;
    private workId: number | undefined;

    constructor(private changeDetector: ChangeDetectorRef, private schedulingService: SchedulingService) {
    }

    async ngOnInit() {
        this.cachedId = this.entry.data.id;
        this.loading = true;
    }

    async ngOnChanges(changes: SimpleChanges) {
        if (changes["entry"] && (this.cachedId === undefined || this.entry.data.id !== this.cachedId)) {
            this.cachedId = this.entry.data.id;
            this.loading = true;
        }
    }

    public ngOnDestroy(): void {
        if (this.workId) {
            this.schedulingService.cancelWork(LOADING_WORK_KEY, this.workId);
        }
    }

    public onClick(): void {
        console.debug(this.entry.data.id);
        this.clickEvent.emit(this);
    }
}
