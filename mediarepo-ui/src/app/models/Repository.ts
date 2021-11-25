export class Repository {
    constructor(
        public name: string,
        public address: string | undefined,
        public path: string | undefined,
        public local: boolean,
    ) {
    }
}
