import {Component, Input, OnInit} from '@angular/core';
import {ProgressSpinnerMode} from "@angular/material/progress-spinner";

@Component({
  selector: 'app-busy-indicator',
  templateUrl: './busy-indicator.component.html',
  styleUrls: ['./busy-indicator.component.scss']
})
export class BusyIndicatorComponent {

  @Input() busy: boolean = false;
  @Input() blurBackground: boolean = false;
  @Input() darkenBackground: boolean = false;
  @Input() mode: ProgressSpinnerMode = "indeterminate";
  @Input() value: number | undefined;

  constructor() { }

  public setBusy(busy: boolean) {
    this.busy = busy;
  }

  public wrapOperation<T>(operation: Function): T | undefined {
    this.setBusy(true)
    try {
      const result = operation();
      this.setBusy(false);
      return result;
    } catch {
      return undefined;
    } finally {
      this.setBusy(false);
    }
  }

  public async wrapAsyncOperation<T>(operation: Function): Promise<T | undefined> {
    this.setBusy(true)
    try {
      const result = await operation();
      this.setBusy(false);
      return result;
    } catch {
      return undefined;
    } finally {
      this.setBusy(false);
    }
  }
}
