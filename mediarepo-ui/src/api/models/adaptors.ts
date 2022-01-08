export function mapOptional<I, O>(mapFn: (value: I) => O): (value: I | undefined) => O | undefined {
    return (value: I | undefined) => value ? mapFn(value) : undefined;
}

export function mapMany<I, O>(mapFn: (value: I) => O): (value: I[]) => O[] {
    return (value: I[]) => value.map(mapFn);
}

export function mapNew<T, V>(classType: new (value: V) => T): (value: V) => T {
    return (value: V) => new classType(value);
}
