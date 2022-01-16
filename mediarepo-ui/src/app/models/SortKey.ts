export class SortKey {


    constructor(
        public sortType: "Namespace" | "FileName" | "FileSize" | "FileImportedTime" | "FileCreatedTime" | "FileChangeTime" | "FileType" | "NumTags",
        public sortDirection: "Ascending" | "Descending",
        public namespaceName: string | undefined
    ) {
    }

    public toString(): string {
        if (this.sortType == "Namespace") {
            return `${this.sortType} '${this.namespaceName}' ${this.sortDirection}`;
        } else {
            return `${this.sortType} ${this.sortDirection}`;
        }
    }

    public toBackendType(): any {

        if (this.sortType == "Namespace") {
            return {
                "Namespace": {
                    direction: this.sortDirection,
                    name: this.namespaceName
                }
            };
        } else {
            let returnObj: any = {};
            returnObj[this.sortType] = this.sortDirection;

            return returnObj;
        }
    }
}
