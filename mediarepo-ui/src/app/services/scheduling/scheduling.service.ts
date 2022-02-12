import {Injectable} from "@angular/core";

@Injectable({
    providedIn: "root"
})
export class SchedulingService {

    private workQueue: { [key: string]: { id: number, cancelled: boolean, cb: Function }[] } = {};
    private lastWorkId = 0;

    constructor() {
    }

    public addWork(key: string, cb: Function): number {
        if (!this.workQueue[key]) {
            this.workQueue[key] = [];
            setTimeout(() => this.startWork(key), 0); // start in the next tick
        }
        const id = this.lastWorkId++;
        this.workQueue[key].push({ id, cb, cancelled: false });
        return id;
    }

    public cancelWork(key: string, id: number) {
        const work = this.workQueue[key]?.find(w => w.id === id);
        if (work) {
            work.cancelled = true;
        }
    }

    public async delay(time: number) {
        return new Promise((res) => {
            setTimeout(res, time);
        });
    }

    private async startWork(key: string) {
        while (true) {
            if (this.workQueue[key]?.length > 0) {
                let work = this.workQueue[key].shift();
                let count = 0;
                while (work?.cancelled && count++ < 100) {
                    work = this.workQueue[key].shift();
                }
                if (work) {
                    try {
                        await work.cb();
                    } catch (err) {
                        console.error(err);
                    }
                }
            }
            await this.delay(0);
        }
    }
}
