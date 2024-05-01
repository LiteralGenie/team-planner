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
