import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FileGalleryEntryComponent } from './file-gallery-entry.component';

describe('FileGalleryEntryComponent', () => {
  let component: FileGalleryEntryComponent;
  let fixture: ComponentFixture<FileGalleryEntryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ FileGalleryEntryComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FileGalleryEntryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
