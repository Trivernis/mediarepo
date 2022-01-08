import {NamespaceData} from "../api-types/tags";

export class Namespace {
    constructor(private data: NamespaceData) {
    }

    public get id(): number {
        return this.data.id;
    }

    public get name(): string {
        return this.data.name;
    }
}
