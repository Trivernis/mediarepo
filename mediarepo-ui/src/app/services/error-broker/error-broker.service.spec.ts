import { TestBed } from '@angular/core/testing';

import { ErrorBrokerService } from './error-broker.service';

describe('ErrorBrokerService', () => {
  let service: ErrorBrokerService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ErrorBrokerService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
