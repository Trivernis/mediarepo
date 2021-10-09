import { Injectable } from '@angular/core';
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class ErrorBrokerService {

  errorCb: Function | undefined;

  constructor() { }

  showError(error: {message: String}) {
    if (this.errorCb) {
      if (!error.message) {
        this.errorCb({message: error});
      } else {
        this.errorCb({...error});
      }
    }
  }
}
