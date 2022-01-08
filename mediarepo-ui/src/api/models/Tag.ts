import {TagData} from "../api-types/tags";

export class Tag {

    private normalizedTag?: string = undefined;

    constructor(
        private tagData: TagData,
    ) {
    }

    public get id(): number {
        return this.tagData.id;
    }

    public get name(): string {
        return this.tagData.name;
    }

    public get namespace(): string | undefined {
        return this.tagData.namespace;
    }

    public getNormalizedOutput(): string {
        if (!this.normalizedTag) {
            this.normalizedTag = this.namespace ? this.namespace + ":" + this.name : this.name;
        }
        return this.normalizedTag;
    }
}
