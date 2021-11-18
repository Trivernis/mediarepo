export class TagQuery {
  constructor(public tag: string, public negate: boolean) {
  }

  public getNormalizedTag(): string {
    return this.negate ? "-" + this.tag : this.tag;
  }
}
