import {Injectable} from "@angular/core";
import {BehaviorSubject} from "rxjs";

@Injectable({
    providedIn: "root"
})
export class TabService {

    public selectedTab = new BehaviorSubject<number>(0);

    constructor() {
    }

    public setSelectedTab(index: number) {
        this.selectedTab.next(index);
    }
}
