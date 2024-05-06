import type { CDragonTrait } from '$lib/types'
import { deepCopy } from '$lib/utils/misc'
import { clone, objectify } from 'radash'
import { getContext, setContext } from 'svelte'
import { writable, type Readable } from 'svelte/store'
import { FILTER_FORM_PARSERS } from './defaults'
import { type FilterForm, type FilterFormControls } from './types'
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

export type FilterFormValue = {
    form: Readable<FilterForm>
    formInitial: Readonly<FilterForm>
    controls: FilterFormControls

    setValue: (value: FilterForm) => void
    destroy: () => void
}

const KEY = 'filter-form'

export function setFilterFormContext(
    initValue: FilterForm,
    traits: CDragonTrait[]
) {
    initValue = deepCopy(initValue)
    const controls = getDefaultControls(initValue, onChange)

    // Init global traits
    const globalTraits = traits.map((trait) => ({
        id: trait.trait_id,
        included: true
    }))
    controls.global.controls.traits.setValue(deepCopy(globalTraits))
    initValue.global.traits = deepCopy(globalTraits)

    // Init slot traits
    const slotTraits = traits.map((trait) => ({
        id: trait.trait_id,
        included: false
    }))
    for (let slot of controls.slots.controls) {
        const traitArray = slot.controls.byAttribute.controls.traits
        traitArray.setValue(deepCopy(slotTraits))
    }
    for (let slot of initValue.slots) {
        slot.byAttribute.traits = deepCopy(slotTraits)
    }

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

        for (let key in update) {
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
    initValue: FilterForm,
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
        Object.entries(initValue),
        ([k, _]) => k as keyof FilterForm,
        ([k, v]) => {
            let key = k as keyof FilterForm
            const parser = FILTER_FORM_PARSERS[key]
            return createControl(v, parser, (val) =>
                // @ts-ignore
                onChange(key, val)
            )
        }
    )
}

export function getFilterFormContext() {
    return getContext(KEY) as FilterFormValue
}
