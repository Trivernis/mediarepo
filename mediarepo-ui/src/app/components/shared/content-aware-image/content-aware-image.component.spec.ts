import {ComponentFixture, TestBed} from '@angular/core/testing';

import {ContentAwareImageComponent} from './content-aware-image.component';

describe('ContentAwareImageComponent', () => {
  let component: ContentAwareImageComponent;
  let fixture: ComponentFixture<ContentAwareImageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
        declarations: [ContentAwareImageComponent]
      })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ContentAwareImageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
