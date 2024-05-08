<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { FormControl } from '$lib/app/form-context/form-control'
    import BoolOption, {
        type OptionData
    } from './bool-option/bool-option.svelte'

    export let slotIndex: number

    const { form, controls } = getFilterFormContext()

    $: ctrl = controls.slots.controls[slotIndex].controls
        .useAttributes as FormControl<boolean>

    $: value = $form.slots[slotIndex].useAttributes

    const options: OptionData[] = [
        {
            label: 'Characteristic',
            id: 'attribute',
            value: true
        },
        {
            label: 'Champion',
            id: 'champion',
            value: false
        }
    ]

    function handleSelect(ev: CustomEvent<boolean>) {
        const val = ev.detail
        ctrl.onChange(val)
    }
</script>

<fieldset class="flex gap-4">
    <legend class="">Filter by:</legend>

    <BoolOption on:select={handleSelect} {value} {options} />
</fieldset>

<style lang="postcss">
    legend {
        float: left;
    }
</style>
