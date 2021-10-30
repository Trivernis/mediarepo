export class Selectable<T> {
  constructor(public data: T, public selected: boolean) {
  }

  public select() {
    this.selected = true;
  }

  public unselect() {
    this.selected = false;
  }

  public toggle() {
    this.selected = !this.selected;
  }
}
