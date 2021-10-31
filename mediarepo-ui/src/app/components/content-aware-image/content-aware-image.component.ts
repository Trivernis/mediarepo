import {Component, Input, OnInit} from '@angular/core';
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
  selector: 'app-content-aware-image',
  templateUrl: './content-aware-image.component.html',
  styleUrls: ['./content-aware-image.component.scss']
})
export class ContentAwareImageComponent {

  @Input() imageSrc!: string | SafeResourceUrl;

  scaleWidth = false;

  constructor() { }

  /**
   * Fits the image into the container
   * @param {HTMLImageElement} image
   * @param {HTMLDivElement} imageContainer
   */
  public adjustSize(image: HTMLImageElement, imageContainer: HTMLDivElement): void {
    const containerHeight = Math.abs(imageContainer.clientHeight);
    const containerWidth = Math.abs(imageContainer.clientWidth);
    const imageRelativeHeight = image.height / containerHeight;
    const imageRelativeWidth = image.width / containerWidth;
    this.scaleWidth = imageRelativeWidth > imageRelativeHeight;
  }
}
