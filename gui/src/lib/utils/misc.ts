import { isEqual, objectify } from 'radash'
import type { Readable, Subscriber, Unsubscriber } from 'svelte/store'

const UNINITIALIZED = Symbol('UNITIALIZED')

// Store that functions like dervied() but ignores duplicate emissions
export class DerivedUniqueStore<T> implements Readable<T> {
    private prevValue: T | typeof UNINITIALIZED = UNINITIALIZED

    constructor(private source: Readable<T>) {}

    subscribe(run: Subscriber<T>): Unsubscriber {
        this.prevValue = UNINITIALIZED

        return this.source.subscribe((update) => {
            if (
                this.prevValue === UNINITIALIZED ||
                !isEqual(this.prevValue, update)
            ) {
                this.prevValue = deepCopy(update)
                run(update)
            }
        })
    }
}

export function deepCopy<T>(x: T): T {
    return JSON.parse(JSON.stringify(x))
}

let next_id = 0

/**
 * UUIDs aren't available on http. This replacement is only meant for testing.
 */
export function getUuidWithFallback(): string {
    const isSsr = typeof window === 'undefined'
    const isHttp = isSsr || window.crypto.randomUUID === undefined
    const hasUuid = !isHttp

    if (hasUuid) {
        return window.crypto.randomUUID()
    } else {
        console.warn(
            'crypto.randomUUID() not available. Falling back to integer ids.'
        )

        const result = (next_id += 1)
        next_id += 1
        return result.toString()
    }
}

export function someFalse(x: Record<string, Boolean>): boolean {
    return Object.values(x).some((v) => v === false)
}

export function filterMap<V, T>(xs: T[], filter: (x: T) => V | null) {
    const mapped = xs.map((x) => filter(x))
    const filtered = mapped.filter((x) => x !== null)
    return filtered as V[]
}

export function objectifyGlobImports(imports: Record<string, any>) {
    return objectify(
        Object.entries(imports),
        // File name minus extension as key
        ([path, val]: [string, any]) => {
            const name = path.split('/').slice(-1)[0]
            const stem = name.split('.')[0]
            return stem
        },
        // Path as value
        ([path, val]: [string, any]) => val.default
    )
}
