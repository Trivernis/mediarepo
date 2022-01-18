import {
    AfterViewChecked,
    ChangeDetectionStrategy,
    ChangeDetectorRef,
    Component,
    ElementRef,
    EventEmitter,
    Input,
    OnChanges,
    OnDestroy,
    OnInit,
    Output,
    SimpleChanges,
    ViewChild
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
export class FileCardComponent implements OnInit, OnChanges, OnDestroy, AfterViewChecked {
    @ViewChild("card") card!: ElementRef;
    @Input() public entry!: Selectable<File>;
    @Input() public fileChanged: BehaviorSubject<void> = new BehaviorSubject<void>(undefined);
    @Output() clickEvent = new EventEmitter<FileCardComponent>();
    @Output() dblClickEvent = new EventEmitter<FileCardComponent>();
    public loading = false;
    private cachedId: number | undefined;
    private workId: number | undefined;
    private selectedPrevious = false;

    constructor(private changeDetector: ChangeDetectorRef, private schedulingService: SchedulingService) {
    }

    async ngOnInit() {
        this.cachedId = this.entry.data.id;
        this.selectedPrevious = this.entry.selected;
        this.setImageDelayed();
    }

    async ngOnChanges(changes: SimpleChanges) {
        if (changes["entry"] && (this.cachedId === undefined || this.entry.data.id !== this.cachedId)) {
            this.cachedId = this.entry.data.id;
            this.setImageDelayed();
        }
        if (changes["fileChanged"]) {
            this.fileChanged.subscribe(() => this.changeDetector.markForCheck());
        }
    }

    public ngAfterViewChecked(): void {
        if (this.entry.selected != this.selectedPrevious) {
            this.selectedPrevious = this.entry.selected;
            this.changeDetector.markForCheck();
        }
    }

    public ngOnDestroy(): void {
        if (this.workId) {
            this.schedulingService.cancelWork(LOADING_WORK_KEY, this.workId);
        }
    }

    private setImageDelayed() {
        if (this.workId) {
            this.schedulingService.cancelWork(LOADING_WORK_KEY, this.workId);
        }
        this.loading = true;
        this.workId = this.schedulingService.addWork(
            LOADING_WORK_KEY,
            async () => {
                await this.schedulingService.delay(1);
                this.loading = false;
                this.changeDetector.markForCheck();
            }
        );
    }
}
