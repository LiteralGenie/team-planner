import { clone, objectify } from 'radash'
import { getContext, setContext } from 'svelte'
import { writable, type Readable } from 'svelte/store'
import { DEFAULT_FILTER_FORM, FILTER_FORM_PARSERS } from './defaults'
import { type FormControlWrapper } from './types'
import { createControl, type ValueOf } from './utils'

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

export interface SlotFilter {
    useAttributes: boolean
    byAttribute: AttributeFilter
    byId: IdFilter
}

export interface FilterForm {
    teamSize: number
    slots: SlotFilter[]
}

export type FilterFormControls = {
    [K in keyof FilterForm]: FormControlWrapper<FilterForm[K]>
}

export type FilterFormValue = {
    form: Readable<FilterForm>
    formInitial: Readonly<FilterForm>
    controls: FilterFormControls

    setValue: (value: FilterForm) => void
    destroy: () => void
}

const KEY = 'filter-form'

export function setFilterFormContext(initValue: FilterForm) {
    const controls = getDefaultControls(onChange)
    const form = writable(clone(initValue))

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
            // @ts-ignore: @todo why ts mad
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
    // Constants for the default and parsers
    // should have already been typechecked at definition
    // and TS cant really infer the key-value relationship here so need to sprinkle ignores
    // @ts-ignore
    return objectify(
        Object.entries(DEFAULT_FILTER_FORM),
        ([k, _]) => k as keyof FilterForm,
        ([k, v]) => {
            let key = k as keyof FilterForm
            const parser = FILTER_FORM_PARSERS[key]
            return createControl(v, parser, (val) =>
                onChange(key, val)
            )
        }
    )
}

export function getFilterFormContext() {
    return getContext(KEY) as FilterFormValue
}
