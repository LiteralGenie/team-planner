<script lang="ts">
    import type {
        AttributeSlotControls,
        AttributeSlotValues
    } from '../form-context/types'
    import type { CheckboxData } from './checkbox-group/checkbox-group.svelte'
    import CheckboxGroup from './checkbox-group/checkbox-group.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    $: options = [
        {
            value: slotValues.range.close,
            label: 'Close',
            onChange: (v: boolean) => handleChange('close', v)
        },

        {
            value: slotValues.range.mid,
            label: 'Mid',
            onChange: (v: boolean) => handleChange('mid', v)
        },

        {
            value: slotValues.range.long,
            label: 'Long',
            onChange: (v: boolean) => handleChange('long', v)
        }
    ] satisfies CheckboxData[]

    function handleChange(
        key: 'close' | 'mid' | 'long',
        value: boolean
    ) {
        const ctrl = slotControls.controls.range.controls[key]
        ctrl.onChange(value)
    }
</script>

<CheckboxGroup label="Range" {options} />
