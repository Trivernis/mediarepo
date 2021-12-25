import {
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
import {File} from "../../../../models/File";
import {Selectable} from "../../../../models/Selectable";
import {
    SchedulingService
} from "../../../../services/scheduling/scheduling.service";

const LOADING_WORK_KEY = "FILE_THUMBNAIL_LOADING";

@Component({
    selector: "app-file-card",
    templateUrl: "./file-card.component.html",
    styleUrls: ["./file-card.component.scss"]
})
export class FileCardComponent implements OnInit, OnChanges, OnDestroy {
    @ViewChild("card") card!: ElementRef;
    @Input() public entry!: Selectable<File>;
    @Output() clickEvent = new EventEmitter<FileCardComponent>();
    @Output() dblClickEvent = new EventEmitter<FileCardComponent>();

    private cachedId: number | undefined;
    private workId: number | undefined;
    public loading = false;

    constructor(private schedulingService: SchedulingService) {
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
        this.workId = this.schedulingService.addWork(LOADING_WORK_KEY,
            async () => {
                await this.schedulingService.delay(1);
                this.loading = false
            });
    }
}
