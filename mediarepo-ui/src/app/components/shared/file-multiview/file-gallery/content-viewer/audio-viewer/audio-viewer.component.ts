import {Component, Input} from '@angular/core';
import {SafeResourceUrl} from "@angular/platform-browser";

@Component({
  selector: 'app-audio-viewer',
  templateUrl: './audio-viewer.component.html',
  styleUrls: ['./audio-viewer.component.scss']
})
export class AudioViewerComponent {

  @Input() blobUrl!: SafeResourceUrl;

  constructor() {
  }
}
