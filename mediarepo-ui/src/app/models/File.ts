export class File {
  constructor(
    public name: string | undefined,
    public comment: string | undefined,
    public hash: string,
    public file_type: number,
    public mime_type: string | undefined,
    public creation_time: Date,
    public change_time: Date,
    public import_time: Date,
  ) {}
}
