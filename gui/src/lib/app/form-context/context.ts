import { clone, isArray } from 'radash'
import { getContext, setContext } from 'svelte'
import { writable, type Readable } from 'svelte/store'
import { FormControl } from './form-control'
import { FormControlArray } from './form-control-array'
import {
    IntParser,
    StringParser,
    type ControlLike,
    type FormParsers,
    type InputParser,
    type ValueOf
} from './utils'

/**
 * This context does a few form-related things...
 *   - Multiple <form>s can be registered. Changes in one form are mirrored to other forms.
 *   - On user interaction (change events), <input> values (strings) are parsed and stored in the context.
 *   - <input> values can be programmatically modified
 *
 * There are a few constraints that can probably be removed if I wasn't lazy...
 *   - The shape of the parsed form data must be flat (no nested objects, no arrays)
 *   - The shape of the parsed form data cannot have optional keys ({ k?: v })
 *   - The parsing logic has to be manually supplied for each field. This context doesn't do any fancy type reflection.
 */

export interface FilterForm {
    teamSize: number
    champions: string[]
}

export type FilterFormValue = {
    form: Readable<FilterForm>
    formInitial: Readonly<FilterForm>
    controls: FilterFormControls

    setValue: (value: FilterForm) => void
    destroy: () => void
}

export type FilterFormControls = Record<
    keyof FilterForm,
    ControlLike<ValueOf<FilterForm>>
>

const KEY = 'filter-form'

export function setFilterFormContext(initValue: FilterForm) {
    // Init controls for each form field
    const controls: FilterFormControls = Object.fromEntries(
        Object.entries(DEFAULT_FILTER_FORM).map(([k, v]) => {
            let key = k as keyof FilterForm
            const parser = FILTER_FORM_PARSERS[
                key
            ] as InputParser<any>

            if (isArray(v)) {
                return [
                    key,
                    new FormControlArray(
                        (vals) => onArrayChange(key, vals),
                        parser,
                        v
                    )
                ]
            } else {
                return [
                    key,
                    new FormControl(
                        (val) => onChange(key, val),
                        parser
                    )
                ]
            }
        })
    ) as any

    // Re-init form on client-side navigation
    const form = writable(clone(initValue))

    // Create context
    const value = {
        form,
        formInitial: initValue,
        controls,
        setValue,
        destroy
    }
    setContext<FilterFormValue>(KEY, value)

    return value

    function onChange(
        key: keyof FilterForm,
        value: ValueOf<FilterForm>
    ) {
        form.update((current) => ({
            ...current,
            [key]: value
        }))
    }

    function onArrayChange(key: keyof FilterForm, vals: string[]) {
        const parser = FILTER_FORM_PARSERS[key]

        let parsed = vals.map((v) => parser.fromString(v))

        form.update((current) => ({
            ...current,
            [key]: parsed
        }))
    }

    function setValue(update: FilterForm) {
        form.set(clone(update))

        for (let key in DEFAULT_FILTER_FORM) {
            const k = key as keyof FilterForm
            const control = controls[k]
            control.setValue(update[k])
        }
    }

    function destroy() {
        for (let c of Object.values(controls)) {
            c.destroy()
        }
    }
}

export function getFilterFormContext() {
    return getContext(KEY) as FilterFormValue
}

export const DEFAULT_FILTER_FORM = {
    teamSize: 7,
    champions: []
} as const satisfies FilterForm

export const FILTER_FORM_PARSERS = {
    teamSize: IntParser,
    champions: StringParser
} as const satisfies FormParsers<FilterForm>
