import {Injectable} from '@angular/core';
import {listen} from "@tauri-apps/api/event";

@Injectable({
  providedIn: 'root'
})
export class ErrorBrokerService {

  errorCb: Function | undefined;
  infoCb: Function | undefined;

  constructor() {
    this.registerListener();
  }

  async registerListener() {
    const _unlisten = await listen("error", event => {
      const payload: any = event.payload;
      if (payload.message) {
        this.showError(payload)
      } else {
        this.showError(payload.toString())
      }
    })
  }

  showInfo(info: string) {
    console.log(info);
    if (this.infoCb) {
      this.infoCb(info);
    }
  }

  showError(error: { message: string }) {
    console.error(error);
    if (this.errorCb) {
      if (!error.message) {
        this.errorCb({message: error});
      } else {
        this.errorCb({...error});
      }
    }
  }
}
