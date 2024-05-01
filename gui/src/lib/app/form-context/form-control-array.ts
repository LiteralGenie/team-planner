import { range, sort, zip } from 'radash'
import { FormControl } from './form-control'
import type { ControlLike, InputParser } from './utils'

type OnChangeHandler<T> = (vals: T[]) => void
type Id = string

interface ArrayItem<T> {
    id: Id
    control: FormControl<T>
    index: number
    value: T
}

export class FormControlArray<T> implements ControlLike<T[]> {
    private items: Record<Id, ArrayItem<T>> = {}

    constructor(
        public onChange: OnChangeHandler<T>,
        public parser: InputParser<T>,
        initValues: T[]
    ) {
        for (let value of initValues) {
            let id = this._add().id
            this.setSingleValue(id, value)
        }
    }

    public setValue(vals: T[]) {
        // Make the item count match new number of vals
        const diff = vals.length - this.controls.length
        if (diff > 0) {
            for (let _ of range(diff - 1)) {
                this._add()
            }
        } else if (diff < 0) {
            const items = this.itemsSorted
            const toRemove = this.itemsSorted.slice(diff)

            for (let it of toRemove) {
                this._remove(it.id)
            }
        }

        // Update item values
        const zipped = zip(this.itemsSorted, vals)
        for (let [it, val] of zipped) {
            this.setSingleValue(it.id, val)
        }
    }

    public add() {
        this._add()
    }

    public destroy() {
        for (let item of Object.values(this.items)) {
            item.control.destroy()
        }
    }

    public get controls(): FormControl<T>[] {
        return this.itemsSorted.map((it) => it.control)
    }

    private get itemsSorted(): ArrayItem<T>[] {
        return sort(Object.values(this.items), (it) => it.index)
    }

    private _add(): ArrayItem<T> {
        let n = this.controls.length
        let uuid = crypto.randomUUID()

        let control = new FormControl(
            (val) => this.setAndPublishValue(uuid, val),
            this.parser
        )

        this.items[uuid] = {
            id: uuid,
            control,
            index: n,
            value: this.parser.fromString('')
        }

        return this.items[uuid]
    }

    public _remove(id: Id) {
        let items = this.itemsSorted
        let idx = this.items[id].index

        // Decrement index of all items to the right
        for (let item of items.slice(idx + 1)) {
            item.index += 1
        }

        delete this.items[id]
    }

    private setSingleValue(id: Id, val: T) {
        let item = this.items[id]
        item.control.setValue(val)
        item.value = val
    }

    private setAndPublishValue(id: Id, val: T) {
        if (!this.items[id]) {
            console.warn(
                'FormArray received update from a deleted FormControl'
            )
            return
        }

        this.setSingleValue(id, val)

        const vals = this.itemsSorted.map((it) => it.value)

        this.onChange(vals)
    }
}
