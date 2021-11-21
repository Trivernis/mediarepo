import {Component, Input, OnInit} from '@angular/core';
import {SafeResourceUrl} from "@angular/platform-browser";

type ContentType = "image" | "video" | "text" | "other";

@Component({
  selector: 'app-content-viewer',
  templateUrl: './content-viewer.component.html',
  styleUrls: ['./content-viewer.component.scss']
})
export class ContentViewerComponent {

  @Input() contentUrl!: SafeResourceUrl | string;
  @Input() mimeType: string | undefined;

  constructor() { }

  public getContentType(): ContentType {
    if (!this.mimeType) {
      return "other";
    }
    let mimeParts = this.mimeType.split("/");
    const type = mimeParts.shift() ?? "other";
    const subtype = mimeParts.shift() ?? "*";

    switch (type) {
      case "image":
        return "image";
      case "video":
        return "video";
      case "text":
        return "text";
      default:
        return "other";
    }
  }
}
