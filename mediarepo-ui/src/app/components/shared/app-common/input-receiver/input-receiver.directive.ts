import {Directive, EventEmitter, HostBinding, HostListener, Output} from "@angular/core";

@Directive({
    selector: "[appInputReceiver]"
})
export class InputReceiverDirective {

    @Output() keyDownEvent = new EventEmitter<KeyboardEvent>();
    @Output() keyUpEvent = new EventEmitter<KeyboardEvent>();
    @HostBinding("tabindex") tabIndex = 1;

    constructor() {
    }

    @HostListener("keydown", ["$event"])
    onKeyDown(event: KeyboardEvent) {
        this.keyDownEvent.emit(event);
    }

    @HostListener("keyup", ["$event"])
    onKeyUp(event: KeyboardEvent) {
        this.keyUpEvent.emit(event);
    }

}
