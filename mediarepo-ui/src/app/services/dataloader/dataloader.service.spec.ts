import { TestBed } from '@angular/core/testing';

import { DataloaderService } from './dataloader.service';

describe('DataloaderService', () => {
  let service: DataloaderService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DataloaderService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
