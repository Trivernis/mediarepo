type CacheEntry<T> = {
    ttl: number,
    value: T,
}

const cacheMap: {
    [key: string]: CacheEntry<any>
} = {};

export class ShortCache {

    public static async cached<T>(
        key: any,
        producer: () => Promise<T>,
        ttl: number = 1000,
        prefix: string = ""
    ): Promise<T> {
        const cacheKey = prefix + JSON.stringify(key);
        const entry = this.getCacheEntry<T>(cacheKey, ttl);

        if (entry) {
            console.debug("cache hit for key", cacheKey);
            return entry;
        } else {
            console.debug("cache miss key", cacheKey);
            const value = await producer();
            this.addCacheEntry(cacheKey, value, ttl);
            return value;
        }
    }

    public static startTicking() {
        (async () => {
            while (true) {
                ShortCache.tick();
                await new Promise(resolve => setTimeout(resolve, 100));
            }
        })();
    }

    private static addCacheEntry(key: string, value: any, ttl: number) {
        cacheMap[key] = {
            ttl,
            value
        };
        console.debug("added cache entry with key", key);
    }

    private static getCacheEntry<T>(key: string, ttl: number): T | undefined {
        const entry = cacheMap[key];
        if (entry) {
            entry.ttl = ttl;
        }
        return entry?.value;
    }

    private static tick() {
        for (let key in cacheMap) {
            cacheMap[key].ttl -= 100;

            if (cacheMap[key].ttl < 0) {
                console.debug("purged cache entry with key", key);
                delete cacheMap[key];
            }
        }
    }
}

ShortCache.startTicking();
