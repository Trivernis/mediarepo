import {ComponentFixture, TestBed} from '@angular/core/testing';

import {FileGridEntryComponent} from './file-grid-entry.component';

describe('FileGridEntryComponent', () => {
  let component: FileGridEntryComponent;
  let fixture: ComponentFixture<FileGridEntryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
        declarations: [FileGridEntryComponent]
      })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FileGridEntryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
