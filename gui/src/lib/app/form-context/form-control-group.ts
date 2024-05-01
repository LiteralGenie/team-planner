import { FormControl } from './form-control'
import type { ControlLike, InputParser, ValueOf } from './utils'

// @todo: FormControlGroups can't be nested or composed with FormControlArrays but it seems possible
type GroupValue = Record<string, string | number>

type ControlGroup<T> = { [K in keyof T]: FormControl<T[K]> }

type ControlParsers<T> = { [K in keyof T]: InputParser<T[K]> }

type OnChangeHandler<T> = (vals: T) => void

export class FormControlGroup<T extends GroupValue>
    implements ControlLike<T>
{
    controls: ControlGroup<T>

    constructor(
        public onChange: OnChangeHandler<T>,
        public parsers: ControlParsers<T>,
        public value: T
    ) {
        // Init controls based on type of value
        // @ts-ignore
        const controls: ControlGroup<T> = {}
        for (let [k, v] of Object.entries(value)) {
            let key = k as keyof T
            const parser = parsers[key]

            if (typeof v === 'string') {
                controls[key] = new FormControl(
                    (update) => this.setAndPublishValue(k, update),
                    parser
                )
            }
        }
        this.controls = controls
    }

    public setValue(x: T) {
        for (let [key, val] of Object.entries(x)) {
            let k = key as keyof T
            let v = val as ValueOf<T>
            this.controls[k].setValue(v)
        }

        this.value = x
    }

    public destroy() {
        for (let control of Object.values(this.controls)) {
            control.destroy()
        }
    }

    private setAndPublishValue(key: keyof T, val: ValueOf<T>) {
        this.value[key] = val
        this.onChange(this.value)
    }
}
