import {Component, Input,} from '@angular/core';
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
  selector: 'app-video-viewer',
  templateUrl: './video-viewer.component.html',
  styleUrls: ['./video-viewer.component.scss']
})
export class VideoViewerComponent {
  @Input() blobUrl!: SafeResourceUrl;
}
