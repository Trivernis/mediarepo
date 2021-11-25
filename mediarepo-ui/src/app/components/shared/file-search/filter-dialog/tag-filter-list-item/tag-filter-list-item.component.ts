import {
  ChangeDetectorRef,
  Component,
  EventEmitter,
  Input,
  OnChanges,
  Output,
  SimpleChanges
} from '@angular/core';
import {
  FilterExpression,
  OrFilterExpression,
  SingleFilterExpression
} from "../../../../../models/FilterExpression";
import {TagQuery} from "../../../../../models/TagQuery";
import {Selectable} from "../../../../../models/Selectable";

@Component({
  selector: 'app-tag-filter-list-item',
  templateUrl: './tag-filter-list-item.component.html',
  styleUrls: ['./tag-filter-list-item.component.scss']
})
export class TagFilterListItemComponent implements OnChanges {

  @Input() expression!: Selectable<FilterExpression>;
  @Output() removeClicked = new EventEmitter<TagFilterListItemComponent>();
  @Output() querySelect = new EventEmitter<TagQuery>();
  @Output() queryUnselect = new EventEmitter<TagQuery>();

  public selectedIndices: number[] = [];

  constructor(private changeDetector: ChangeDetectorRef) {
  }

  public ngOnChanges(changes: SimpleChanges): void {
    if (changes["expression"]) {
      this.selectedIndices = [];
    }
  }

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

  public selectInnerIndex(index: number): void {
    const expression = this.expression.data as OrFilterExpression;

    if (this.selectedIndices.includes(index)) {
      const elementIndex = this.selectedIndices.indexOf(index);
      this.selectedIndices.splice(elementIndex, 1);
      this.queryUnselect.emit(expression.filter[index]);
    } else {
      this.selectedIndices.push(index);
      this.querySelect.emit(expression.filter[index]);
    }
  }

  public onSelect(): void {
    this.expression.selected = !this.expression.selected;
    if (this.expression.selected) {
      this.querySelect.emit(this.expression.data.filter as TagQuery);
    } else {
      this.queryUnselect.emit(this.expression.data.filter as TagQuery);
    }
  }
}
