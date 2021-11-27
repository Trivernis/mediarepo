import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FileImportComponent } from './file-import.component';

describe('FileImportComponent', () => {
  let component: FileImportComponent;
  let fixture: ComponentFixture<FileImportComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ FileImportComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FileImportComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
