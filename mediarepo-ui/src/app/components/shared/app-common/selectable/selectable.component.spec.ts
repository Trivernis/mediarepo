import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SelectableComponent } from './selectable.component';

describe('SelectableComponent', () => {
  let component: SelectableComponent;
  let fixture: ComponentFixture<SelectableComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SelectableComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(SelectableComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
