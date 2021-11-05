import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FilesTabComponent } from './files-tab.component';

describe('SearchPageComponent', () => {
  let component: FilesTabComponent;
  let fixture: ComponentFixture<FilesTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ FilesTabComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FilesTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
