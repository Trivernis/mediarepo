export type TagData = {
    id: number,
    namespace?: string,
    name: string,
};

export type NamespaceData = {
    id: number,
    name: string,
};

export type CdTagMappings = {
    [key: string]: TagData[],
};
