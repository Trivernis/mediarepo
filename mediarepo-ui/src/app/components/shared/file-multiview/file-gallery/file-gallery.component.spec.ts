import {ComponentFixture, TestBed} from '@angular/core/testing';

import {FileGalleryComponent} from './file-gallery.component';

describe('FileGalleryComponent', () => {
  let component: FileGalleryComponent;
  let fixture: ComponentFixture<FileGalleryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
        declarations: [FileGalleryComponent]
      })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FileGalleryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
