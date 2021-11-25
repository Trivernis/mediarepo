import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ImportTabComponent } from './import-tab.component';

describe('ImportTabComponent', () => {
  let component: ImportTabComponent;
  let fixture: ComponentFixture<ImportTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ImportTabComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ImportTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
