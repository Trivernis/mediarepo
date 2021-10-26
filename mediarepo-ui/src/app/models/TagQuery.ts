export class TagQuery {
  constructor(public name: string, public negate: boolean) {
  }

  public getNormalizedTag(): string {
    return this.negate? "-" + this.name : this.name;
  }
}
