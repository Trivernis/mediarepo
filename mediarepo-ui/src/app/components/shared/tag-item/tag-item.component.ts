import {Component, Input} from '@angular/core';
import {Tag} from "../../../models/Tag";

@Component({
  selector: 'app-tag-item',
  templateUrl: './tag-item.component.html',
  styleUrls: ['./tag-item.component.scss']
})
export class TagItemComponent {

  @Input() tag!: Tag;
  @Input() namespaceColor: string | undefined;
  @Input() tagColor: string | undefined;

  constructor() {
  }
}
