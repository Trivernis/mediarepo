import {ComponentFixture, TestBed} from '@angular/core/testing';

import {TagFilterListItemComponent} from './tag-filter-list-item.component';

describe('TagFilterListItemComponent', () => {
  let component: TagFilterListItemComponent;
  let fixture: ComponentFixture<TagFilterListItemComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
        declarations: [TagFilterListItemComponent]
      })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(TagFilterListItemComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
