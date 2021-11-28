import { ComponentFixture, TestBed } from '@angular/core/testing';

import { EditableMetadataEntryComponent } from './editable-metadata-entry.component';

describe('EditableMetadataEntryComponent', () => {
  let component: EditableMetadataEntryComponent;
  let fixture: ComponentFixture<EditableMetadataEntryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ EditableMetadataEntryComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(EditableMetadataEntryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
