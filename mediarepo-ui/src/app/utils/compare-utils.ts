export function compareSearchResults(query: string, l: string, r: string): number {
    if (l.startsWith(query) && !r.startsWith(query)) {
        return -1;
    } else if (!l.startsWith(query) && r.startsWith(query)) {
        return 1;
    } else if (l.length < r.length) {
        return -1;
    } else if (l.length > r.length) {
        return 1;
    } else {
        return l.localeCompare(r);
    }
}
