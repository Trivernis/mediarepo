import {FileBasicData, FileStatus} from "../api-types/files";

export class File {
    constructor(
        private basicData: FileBasicData,
    ) {
    }

    public get rawData(): FileBasicData {
        return this.basicData;
    }

    public get id(): number {
        return this.basicData.id;
    }

    public get cd(): string {
        return this.basicData.cd;
    }

    public get status(): FileStatus {
        return this.basicData.status;
    }

    public set status(value: FileStatus) {
        this.basicData.status = value;
    }

    public get mimeType(): string {
        return this.basicData.mime_type;
    }
}
