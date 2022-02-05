import {RepositoryData} from "../api-types/repo";

export class Repository {
    constructor(
        private repoData: RepositoryData,
    ) {
    }

    public get name(): string {
        return this.repoData.name;
    }

    public get address(): string | undefined {
        return this.repoData.address;
    }

    public get path(): string | undefined {
        return this.repoData.path;
    }

    public get local(): boolean {
        return this.repoData.local;
    }

    public update(data: { name?: string, address?: string, path?: string, local?: boolean }) {
        this.repoData = Object.assign(this.repoData, data);
    }
}
