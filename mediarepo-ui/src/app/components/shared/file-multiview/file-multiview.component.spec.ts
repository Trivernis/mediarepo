import {ComponentFixture, TestBed} from '@angular/core/testing';

import {FileMultiviewComponent} from './file-multiview.component';

describe('FileMultiviewComponent', () => {
  let component: FileMultiviewComponent;
  let fixture: ComponentFixture<FileMultiviewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
        declarations: [FileMultiviewComponent]
      })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FileMultiviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
