import { deepCopy } from '$lib/utils/misc'
import { objectify } from 'radash'
import { getContext, setContext } from 'svelte'
import { get, writable, type Readable } from 'svelte/store'
import {
    DEFAULT_GLOBAL_FILTER,
    DEFAULT_SLOT_FILTER,
    FILTER_FORM_PARSERS
} from './defaults'
import { type FilterForm, type FilterFormControls } from './types'
import {
    createControl,
    serializeFilterForm,
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

export type FilterFormValue = {
    form: Readable<FilterForm>
    formInitial: Readable<Readonly<FilterForm>>
    controls: FilterFormControls

    submit: () => void
    setValue: (value: FilterForm) => void
    resetGlobalFilter: () => void
    resetSlotFilter: (idx: number) => void
    destroy: () => void
}

const KEY = 'filter-form'

export function setFilterFormContext(initValue: FilterForm) {
    const formInitial = writable(deepCopy(initValue))
    const controls = getDefaultControls(initValue, onChange)

    const form = writable(deepCopy(initValue))

    const value = {
        form,
        formInitial,
        controls,
        submit,
        setValue,
        resetGlobalFilter,
        resetSlotFilter,
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

    function resetSlotFilter(idx: number) {
        const slotsUpdate = [...get(form).slots]

        slotsUpdate[idx] = {
            ...deepCopy(DEFAULT_SLOT_FILTER),
            useAttributes: slotsUpdate[idx].useAttributes
        }

        setValue({
            ...get(form),
            slots: slotsUpdate
        })
    }

    function resetGlobalFilter() {
        const current = deepCopy(get(form))

        setValue({
            ...current,
            global: deepCopy(DEFAULT_GLOBAL_FILTER)
        })
    }

    function setValue(update: FilterForm) {
        form.set(deepCopy(update))

        for (let key in update) {
            const k = key as keyof FilterForm
            const control = controls[k]
            // @ts-ignore: @todo why ts mad
            control.setValue(update[k])
        }
    }

    function submit() {
        const formData = get(form)
        formInitial.set(formData)

        const update = new URL(window.location.href)
        update.search = ''
        update.searchParams.set(
            'query',
            serializeFilterForm(formData)
        )
        // replaceState(update, {})
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
