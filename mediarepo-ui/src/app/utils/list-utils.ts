export function enumerate<T>(list: T[]): [number, T][] {
    const enumeratedEntries = [];

    for (let i = 0; i < list.length; i++) {
        enumeratedEntries.push([i, list[i]] as [number, T]);
    }
    return enumeratedEntries;
}

export function removeByValue<T>(list: T[], entry: T) {
    const index = list.indexOf(entry);
    if (index >= 0) {
        list.splice(index, 1);
    }
}
