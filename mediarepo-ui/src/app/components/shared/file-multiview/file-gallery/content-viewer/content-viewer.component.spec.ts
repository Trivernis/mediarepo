import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ContentViewerComponent } from './content-viewer.component';

describe('ContentViewerComponent', () => {
  let component: ContentViewerComponent;
  let fixture: ComponentFixture<ContentViewerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ContentViewerComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ContentViewerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
