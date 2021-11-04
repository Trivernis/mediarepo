export class Tag {

  private normalizedTag?: string = undefined;

  constructor(
    public id: number,
    public name: string,
    public namespace: string | undefined
  ) {}

  public getNormalizedOutput(): string {
    if (!this.normalizedTag) {
      this.normalizedTag = this.namespace ? this.namespace + ':' + this.name : this.name
    }
    return this.normalizedTag;
  }
};
