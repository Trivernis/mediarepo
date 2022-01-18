import {BehaviorSubject} from "rxjs";

export class Selectable<T> {

    public selected: BehaviorSubject<boolean>;

    constructor(public data: T, selected: boolean) {
        this.selected = new BehaviorSubject<boolean>(selected);
    }

    public select() {
        this.selected.next(true);
    }

    public unselect() {
        this.selected.next(false);
    }
}
