import {Injectable} from "@angular/core";
import {MediarepoApi} from "../../../api/Api";
import {JobType} from "../../../api/api-types/job";

@Injectable({
    providedIn: "root"
})
export class JobService {

    constructor() {
    }

    public async runJob(jobType: JobType, sync: boolean = true): Promise<void> {
        return MediarepoApi.runJob({ jobType, sync });
    }

    public async isJobRunning(jobType: JobType): Promise<boolean> {
        return MediarepoApi.isJobRunning({ jobType });
    }
}
