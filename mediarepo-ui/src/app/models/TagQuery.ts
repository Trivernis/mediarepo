export class TagQuery {
    constructor(public tag: string, public negate: boolean) {
    }

    public static fromString(tag: string): TagQuery {
        if (tag.startsWith("-")) {
            return new TagQuery(tag.replace(/^-/g, ""), true);
        } else {
            return new TagQuery(tag, false);
        }
    }

    public getNormalizedTag(): string {
        return this.negate ? "-" + this.tag : this.tag;
    }
}
