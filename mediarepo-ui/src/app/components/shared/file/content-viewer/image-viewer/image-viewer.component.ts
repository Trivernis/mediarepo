import {
    Component,
    HostListener,
    Input,
    OnChanges,
    SimpleChanges
} from "@angular/core";
import {CdkDragMove} from "@angular/cdk/drag-drop";
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
    selector: "app-image-viewer",
    templateUrl: "./image-viewer.component.html",
    styleUrls: ["./image-viewer.component.scss"]
})
export class ImageViewerComponent implements OnChanges {
    @Input() imageUrl!: SafeResourceUrl | string;
    public imageZoom = 1;
    public imagePosition = {x: 0, y: 0};
    public mouseInImageView = false;

    constructor() {
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["imageUrl"]) {
            this.resetImage();
        }
    }


    public resetImage() {
        this.imageZoom = 1;
        this.imagePosition = {x: 0, y: 0};
    }

    public onDragMoved($event: CdkDragMove<HTMLDivElement>): void {
        this.imagePosition.x += $event.delta.x;
        this.imagePosition.y += $event.delta.y;
    }

    @HostListener("window:keydown", ["$event"])
    private async handleKeydownEvent(event: KeyboardEvent) {
        switch (event.key) {
            case "Escape":
                this.resetImage();
                break;
        }
    }

    @HostListener("mousewheel", ["$event"])
    private handleScroll(event: any) {
        if (this.mouseInImageView) {
            const delta = event.wheelDelta ?? event.detail;

            if (delta > 0) {
                this.imageZoom += 0.2;
                if (this.imageZoom > 4) {
                    this.imageZoom = 4;
                }
            } else if (delta < 0) {
                this.imageZoom -= 0.2;
                if (this.imageZoom < 0.5) {
                    this.imageZoom = 0.5;
                }
            }
        }
    }
}
