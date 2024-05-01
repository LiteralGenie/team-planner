import { clone, isArray, isObject, range } from 'radash'
import { getContext, setContext } from 'svelte'
import { writable, type Readable } from 'svelte/store'
import { FormControl } from './form-control'
import { FormControlArray } from './form-control-array'
import { FormControlGroup } from './form-control-group'
import {
    BoolParser,
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
 *   - The shape of the parsed form data cannot have optional keys ({ k?: v })
 *   - The parsing logic has to be manually supplied for each field. This context doesn't do any fancy type reflection.
 */

export type RangeAttribute = 'close' | 'mid' | 'far'

export type DamageTypeAttribute = 'ap' | 'ad'

export interface AttributeFilter {
    /** 1,2,3,4,5 */
    cost: [Boolean, Boolean, Boolean, Boolean, Boolean]
    /** close, mid, far */
    range: [Boolean, Boolean, Boolean]
    traitIdsExcluded: string[]
    /** ad, ap */
    damageType: [Boolean, Boolean]
}

export type IdFilter = string[]

export interface FilterForm {
    teamSize: number
    slotsByAttribute: AttributeFilter[]
    slotsById: IdFilter[]
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
    const controls = getDefaultControls(onChange)

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

function getDefaultControls(
    onChange: (
        key: keyof FilterForm,
        value: ValueOf<FilterForm>
    ) => void
): FilterFormControls {
    return Object.fromEntries(
        Object.entries(DEFAULT_FILTER_FORM).map(([k, v]) => {
            let key = k as keyof FilterForm

            if (isArray(v)) {
                const parser = FILTER_FORM_PARSERS[
                    key
                ] as InputParser<any>

                return [
                    key,
                    new FormControlArray(
                        (vals) => onChange(key, vals),
                        parser,
                        v
                    )
                ]
            }
            if (isObject(v)) {
                const parser = FILTER_FORM_PARSERS[key] as Record<
                    string,
                    InputParser<any>
                >

                return [
                    key,
                    new FormControlGroup(
                        (val) => onChange(key, val),
                        // @fixme: ts is inferring never here
                        // @ts-ignore
                        parser as any,
                        v
                    )
                ]
            } else {
                const parser = FILTER_FORM_PARSERS[
                    key
                ] as InputParser<any>

                return [
                    key,
                    new FormControl(
                        (val) => onChange(key, val),
                        parser
                    )
                ]
            }
        })
    )
}

export function getFilterFormContext() {
    return getContext(KEY) as FilterFormValue
}

const DEFAULT_ATTRIBUTE_SLOT = {
    cost: [true, true, true, true, true],
    range: [true, true, true],
    traitIdsExcluded: [],
    damageType: [true, true]
} satisfies AttributeFilter

export const DEFAULT_FILTER_FORM = {
    teamSize: 7,
    slotsByAttribute: [...range(7)].map((_) =>
        clone(DEFAULT_ATTRIBUTE_SLOT)
    ),
    slotsById: [[], [], [], [], [], [], []]
} as const satisfies FilterForm

export const FILTER_FORM_PARSERS = {
    teamSize: IntParser,
    slotsByAttribute: {
        // @todo: enum parsers
        cost: BoolParser,
        range: StringParser as any,
        traitIdsExcluded: StringParser,
        damageType: StringParser as any
    },
    slotsById: StringParser
} as const satisfies FormParsers<FilterForm>
