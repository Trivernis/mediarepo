import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FileContextMenuComponent } from './file-context-menu.component';

describe('FileContextMenuComponent', () => {
  let component: FileContextMenuComponent;
  let fixture: ComponentFixture<FileContextMenuComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ FileContextMenuComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FileContextMenuComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
