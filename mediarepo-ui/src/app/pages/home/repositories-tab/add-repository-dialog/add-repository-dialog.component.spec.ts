import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddRepositoryDialogComponent } from './add-repository-dialog.component';

describe('AddRepositoryDialogComponent', () => {
  let component: AddRepositoryDialogComponent;
  let fixture: ComponentFixture<AddRepositoryDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AddRepositoryDialogComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(AddRepositoryDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
