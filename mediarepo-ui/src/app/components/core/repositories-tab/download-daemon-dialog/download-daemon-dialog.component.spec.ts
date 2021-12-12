import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DownloadDaemonDialogComponent } from './download-daemon-dialog.component';

describe('DownloadDaemonDialogComponent', () => {
  let component: DownloadDaemonDialogComponent;
  let fixture: ComponentFixture<DownloadDaemonDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DownloadDaemonDialogComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(DownloadDaemonDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
