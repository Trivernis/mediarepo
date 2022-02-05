import {SortDirection, SortKeyData} from "../api-types/files";

export type SortType =
    "Namespace"
    | "FileName"
    | "FileSize"
    | "FileImportedTime"
    | "FileCreatedTime"
    | "FileChangeTime"
    | "FileType"
    | "NumTags";

export class SortKey {

    constructor(private data: SortKeyData) {
        this.data = data;
    }

    public get sortType(): SortType {
        return Reflect.ownKeys(this.data)[0] as SortType;
    }

    public set sortType(value: SortType) {
        if (value == "Namespace") {
            this.data = {
                Namespace: {
                    direction: this.sortDirection,
                    name: ""
                }
            };
        } else {
            this.data = {
                [value]: {
                    direction: this.sortDirection
                }
            } as SortKeyData;
        }
    }

    public get sortDirection(): SortDirection {
        if ("Namespace" in this.data) {
            return this.data.Namespace.direction;
        } else {
            // @ts-ignore
            return this.data[this.sortType];
        }
    }

    public set sortDirection(value: SortDirection) {
        const sortType = this.sortType;
        if ("Namespace" in this.data) {
            this.data.Namespace.direction = value;
        } else {
            // @ts-ignore
            this.data[this.sortType] = value;
        }
    }

    public get namespaceName(): string | undefined {
        if ("Namespace" in this.data) {
            return this.data.Namespace.name;
        }
        return undefined;
    }

    public set namespaceName(value: string | undefined) {
        if (value && "Namespace" in this.data) {
            this.data.Namespace.name = value;
        }
    }

    public static fromValues(
        sortType: SortType,
        sortDirection: SortDirection,
        namespaceName: string | undefined
    ) {
        let data;
        
        if (sortType === "Namespace") {
            data = {
                Namespace: {
                    name: namespaceName!,
                    direction: sortDirection
                }
            };
        } else {
            data = {
                [sortType]: sortDirection
            } as SortKeyData;
        }

        return new SortKey(data);
    }

    public toString(): string {
        if (this.sortType == "Namespace") {
            return `${this.sortType} '${this.namespaceName}' ${this.sortDirection}`;
        } else {
            return `${this.sortType} ${this.sortDirection}`;
        }
    }

    public rawData(): SortKeyData {
        return this.data;
    }
}
