import {Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {FileService} from "../../services/file/file.service";
import {File} from "../../models/File";
import {Lightbox, LIGHTBOX_EVENT, LightboxEvent} from "ngx-lightbox";
import {ErrorBrokerService} from "../../services/error-broker/error-broker.service";
import {TagService} from "../../services/tag/tag.service";
import {Tag} from "../../models/Tag";
import {MatChipInputEvent} from "@angular/material/chips";
import {COMMA, ENTER} from "@angular/cdk/keycodes";
import {MatSelectionListChange} from "@angular/material/list";
import {MatAutocompleteSelectedEvent} from "@angular/material/autocomplete";
import {Observable} from "rxjs";
import {map, startWith} from "rxjs/operators";
import {FormControl} from "@angular/forms";
import {FileSearchComponent} from "../../components/file-search/file-search.component";
import {DataloaderService} from "../../services/dataloader/dataloader.service";

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
  public async ngOnInit(): Promise<void> {
    await this.dataloaderService.loadData();
  }


  constructor(private dataloaderService: DataloaderService) {}
}
