import { TestBed } from '@angular/core/testing';

import { SchedulingService } from './scheduling.service';

describe('SchedulingService', () => {
  let service: SchedulingService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SchedulingService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
