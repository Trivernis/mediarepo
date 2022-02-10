import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-drawer-page-content',
  templateUrl: './drawer-page-content.component.html',
  styleUrls: ['./drawer-page-content.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DrawerPageContentComponent implements OnInit {

  constructor() { }

  ngOnInit(): void {
  }

}
