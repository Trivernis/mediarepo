import { TestBed } from '@angular/core/testing';

import { PresetService } from './preset.service';

describe('PresetService', () => {
  let service: PresetService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(PresetService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
