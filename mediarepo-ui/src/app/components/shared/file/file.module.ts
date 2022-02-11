import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FileMultiviewComponent} from "./file-multiview/file-multiview.component";
import {FileGridComponent} from "./file-multiview/file-grid/file-grid.component";
import {FileGalleryComponent} from "./file-multiview/file-gallery/file-gallery.component";
import {FileCardComponent} from "./file-card/file-card.component";
import {FileContextMenuComponent} from "./file-context-menu/file-context-menu.component";
import {FileThumbnailComponent} from "./file-thumbnail/file-thumbnail.component";
import {ContentViewerComponent} from "./content-viewer/content-viewer.component";
import {AudioViewerComponent} from "./content-viewer/audio-viewer/audio-viewer.component";
import {ImageViewerComponent} from "./content-viewer/image-viewer/image-viewer.component";
import {VideoViewerComponent} from "./content-viewer/video-viewer/video-viewer.component";
import {AppCommonModule} from "../app-common/app-common.module";
import {MatSliderModule} from "@angular/material/slider";
import {NgIconsModule} from "@ng-icons/core";
import {
    MatAudiotrack,
    MatAutoDelete,
    MatClose,
    MatDescription,
    MatExpandLess,
    MatExpandMore,
    MatFiberNew,
    MatGif,
    MatImage,
    MatMovie,
    MatRefresh
} from "@ng-icons/material-icons";
import {DragDropModule} from "@angular/cdk/drag-drop";
import {MatButtonModule} from "@angular/material/button";
import {MatMenuModule} from "@angular/material/menu";
import {MatDividerModule} from "@angular/material/divider";
import {FlexModule} from "@angular/flex-layout";
import {ScrollingModule} from "@angular/cdk/scrolling";
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {MatCardModule} from "@angular/material/card";


@NgModule({
    declarations: [
        FileMultiviewComponent,
        FileGridComponent,
        FileGalleryComponent,
        FileCardComponent,
        FileContextMenuComponent,
        FileThumbnailComponent,
        ContentViewerComponent,
        AudioViewerComponent,
        ImageViewerComponent,
        VideoViewerComponent,
    ],
    exports: [
        FileMultiviewComponent
    ],
    imports: [
        CommonModule,
        AppCommonModule,
        MatSliderModule,
        NgIconsModule.withIcons({
            MatRefresh,
            MatClose,
            MatImage,
            MatMovie,
            MatGif,
            MatAudiotrack,
            MatDescription,
            MatAutoDelete,
            MatFiberNew,
            MatExpandMore,
            MatExpandLess
        }),
        DragDropModule,
        MatButtonModule,
        MatMenuModule,
        MatDividerModule,
        FlexModule,
        ScrollingModule,
        MatProgressSpinnerModule,
        MatCardModule
    ]
})
export class FileModule {
}
