import {
  ChangeDetectorRef,
  Component,
  EventEmitter,
  Inject,
  Input,
  OnInit,
  Output
} from '@angular/core';
import {
  FilterExpression,
  OrFilterExpression, SingleFilterExpression
} from "../../../../models/FilterExpression";
import {TagQuery} from "../../../../models/TagQuery";
import {Selectable} from "../../../../models/Selectable";

@Component({
  selector: 'app-tag-filter-list-item',
  templateUrl: './tag-filter-list-item.component.html',
  styleUrls: ['./tag-filter-list-item.component.scss']
})
export class TagFilterListItemComponent {

  @Input() expression!: Selectable<FilterExpression>;
  @Output() removeClicked = new EventEmitter<TagFilterListItemComponent>();

  constructor() { }

  public enumerate<T>(items: T[]): [number, T][] {
    return items.map((value, index) => [index, value]);
  }

  public removeOrExpression(index: number) {
    const expression = this.expression.data as OrFilterExpression;
    expression.removeQueryEntry(index);

    if (expression.filter.length == 0) {
      this.removeClicked.emit(this);
    } else if (expression.filter.length == 1) {
      this.expression.data = new SingleFilterExpression(expression.filter[0]);
    }
  }
}
