import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-middle-centered',
  templateUrl: './middle-centered.component.html',
  styleUrls: ['./middle-centered.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MiddleCenteredComponent implements OnInit {

  constructor() { }

  ngOnInit(): void {
  }

}
