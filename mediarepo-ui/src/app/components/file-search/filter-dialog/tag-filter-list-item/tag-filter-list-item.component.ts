import {Component, Input, OnInit} from '@angular/core';
import {FilterExpression} from "../../../../models/FilterExpression";

@Component({
  selector: 'app-tag-filter-list-item',
  templateUrl: './tag-filter-list-item.component.html',
  styleUrls: ['./tag-filter-list-item.component.scss']
})
export class TagFilterListItemComponent {

  @Input() expression!: FilterExpression;

  constructor() { }

  ngOnInit(): void {
  }

}
