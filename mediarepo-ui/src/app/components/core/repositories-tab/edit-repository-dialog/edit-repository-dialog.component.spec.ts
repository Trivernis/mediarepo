import { ComponentFixture, TestBed } from '@angular/core/testing';

import { EditRepositoryDialogComponent } from './edit-repository-dialog.component';

describe('EditRepositoryDialogComponent', () => {
  let component: EditRepositoryDialogComponent;
  let fixture: ComponentFixture<EditRepositoryDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ EditRepositoryDialogComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(EditRepositoryDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
