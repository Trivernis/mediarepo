import {FileBasicData, FileStatus} from "../api-types/files";
import {BehaviorSubject, Observable} from "rxjs";

export class File {

    private statusSubject: BehaviorSubject<FileStatus>;

    constructor(
        private basicData: FileBasicData,
    ) {
        this.statusSubject = new BehaviorSubject(basicData.status);
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

    public get status(): Observable<FileStatus> {
        return this.statusSubject.asObservable();
    }

    public get mimeType(): string {
        return this.basicData.mime_type;
    }

    public setStatus(value: FileStatus) {
        this.basicData.status = value;
        this.statusSubject.next(value);
    }

    public getStatus(): FileStatus {
        return this.basicData.status;
    }
}
