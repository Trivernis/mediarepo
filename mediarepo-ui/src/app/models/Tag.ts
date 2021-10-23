export class Tag {
  constructor(
    public id: number,
    public name: string,
    public namespace: string | undefined
  ) {}

  public getNormalizedOutput(): string {
    return this.namespace ? this.namespace + ':' + this.name : this.name
  }
};
