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
import {BehaviorSubject} from "rxjs";

const LOADING_WORK_KEY = "FILE_THUMBNAIL_LOADING";

@Component({
    selector: "app-file-card",
    templateUrl: "./file-card.component.html",
    styleUrls: ["./file-card.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FileCardComponent implements OnInit, OnChanges, OnDestroy {

    @Input() public entry!: Selectable<File>;
    @Input() public fileChanged: BehaviorSubject<void> = new BehaviorSubject<void>(undefined);
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
        if (changes["fileChanged"]) {
            this.fileChanged.subscribe(() => this.changeDetector.markForCheck());
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

    private setImageDelayed() {
        if (this.workId) {
            this.schedulingService.cancelWork(LOADING_WORK_KEY, this.workId);
        }
        this.loading = true;
        this.workId = this.schedulingService.addWork(
            LOADING_WORK_KEY,
            async () => {
                await this.schedulingService.delay(0);
                this.loading = false;
                this.changeDetector.markForCheck();
            }
        );
    }
}
