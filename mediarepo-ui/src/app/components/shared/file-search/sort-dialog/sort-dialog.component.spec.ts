import {ComponentFixture, TestBed} from '@angular/core/testing';

import {SortDialogComponent} from './sort-dialog.component';

describe('FilterDialogComponent', () => {
  let component: SortDialogComponent;
  let fixture: ComponentFixture<SortDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
        declarations: [SortDialogComponent]
      })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(SortDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
