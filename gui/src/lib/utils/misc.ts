import { isEqual } from 'radash'
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
                this.prevValue = update
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
