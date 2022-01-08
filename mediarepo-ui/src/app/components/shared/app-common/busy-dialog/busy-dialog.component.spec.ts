import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BusyDialogComponent } from './busy-dialog.component';

describe('BusyDialogComponent', () => {
  let component: BusyDialogComponent;
  let fixture: ComponentFixture<BusyDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BusyDialogComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BusyDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
