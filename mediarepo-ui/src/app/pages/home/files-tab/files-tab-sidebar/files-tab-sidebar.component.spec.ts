import {ComponentFixture, TestBed} from '@angular/core/testing';

import {FilesTabSidebarComponent} from './files-tab-sidebar.component';

describe('FilesTabSidebarComponent', () => {
  let component: FilesTabSidebarComponent;
  let fixture: ComponentFixture<FilesTabSidebarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
        declarations: [FilesTabSidebarComponent]
      })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FilesTabSidebarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
