/**
 * Normalizes the tag by removing whitespaces and enforcing lowercase
 * @param {string} tag
 * @returns {string}
 * @private
 */
export function normalizeTag(tag: string): string {
    let normalizedTag = tag.trim().toLowerCase();
    let parts = normalizedTag.split(":");

    if (parts.length > 1) {
        const namespace = parts.shift()!.trim();
        const name = parts.join(":").trim();
        return namespace + ":" + name;
    } else {
        return normalizedTag;
    }
}
