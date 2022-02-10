import {Component, DoCheck, ElementRef, Input, OnInit, ViewChild} from "@angular/core";
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
    selector: "app-content-aware-image",
    templateUrl: "./content-aware-image.component.html",
    styleUrls: ["./content-aware-image.component.scss"]
})
export class ContentAwareImageComponent implements OnInit, DoCheck {
    @Input() imageSrc!: string | SafeResourceUrl;
    @Input() maximizeHeight: boolean = true;
    @Input() maximizeWidth: boolean = true;
    @Input() borderRadius: string | undefined;
    @Input() decoding: "async" | "sync" | "auto" = "auto";
    @ViewChild("image") image?: ElementRef<HTMLImageElement>;
    @ViewChild("imageContainer") imageContainer?: ElementRef<HTMLDivElement>;
    scaleWidth = false;
    private previousHeight = 0;
    private previousWidth = 0;

    constructor() {
    }

    public ngOnInit(): void {
        if (this.image) {
            this.image.nativeElement.decoding = this.decoding;
        }
    }

    public ngDoCheck(): void {
        if (this.image?.nativeElement && this.imageContainer?.nativeElement) {
            this.adjustSize(this.image.nativeElement, this.imageContainer.nativeElement);
        }
    }

    /**
     * Fits the image into the container
     * @param {HTMLImageElement} image
     * @param {HTMLDivElement} imageContainer
     */
    public adjustSize(image: HTMLImageElement, imageContainer: HTMLDivElement): void {
        const containerHeight = Math.abs(imageContainer.clientHeight);
        const containerWidth = Math.abs(imageContainer.clientWidth);

        if (this.previousWidth != containerWidth || this.previousHeight != containerHeight) {
            this.previousHeight = containerHeight;
            this.previousWidth = containerWidth;
            const imageRelativeHeight = image.naturalHeight / containerHeight;
            const imageRelativeWidth = image.naturalWidth / containerWidth;
            this.scaleWidth = imageRelativeWidth > imageRelativeHeight;
        }
    }
}
