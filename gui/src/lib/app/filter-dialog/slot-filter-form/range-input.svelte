<script lang="ts">
    import type {
        AttributeSlotControls,
        AttributeSlotValues
    } from '$lib/app/form-context/types'
    import RangeIcon from '$lib/icons/range-icon.svelte'
    import type { CheckboxData } from '../checkbox-group/checkbox-group.svelte'
    import CheckboxGroup from '../checkbox-group/checkbox-group.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    $: options = [
        {
            value: slotValues.range.close,
            label: 'Melee',
            tooltip: 'Melee units have a 1-hex range',
            onChange: (v: boolean) => handleChange('close', v),
            prefix: RangeIcon,
            prefixOpts: { activeClose: true },
            prefixClass: 'h-4 w-4'
        },

        {
            value: slotValues.range.mid,
            label: 'Mid',
            tooltip: 'Mid-range units have a 2-hex range',
            onChange: (v: boolean) => handleChange('mid', v),
            prefix: RangeIcon,
            prefixOpts: { activeMid: true },
            prefixClass: 'h-4 w-4'
        },

        {
            value: slotValues.range.long,
            label: 'Long',
            tooltip: 'Long-range units have a range of 3+ hexes',
            onChange: (v: boolean) => handleChange('long', v),
            prefix: RangeIcon,
            prefixOpts: { activeLong: true },
            prefixClass: 'h-4 w-4'
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

<div class="root">
    <CheckboxGroup label="Range" {options} />
</div>

<style lang="postcss">
    .root :global(.range-icon-root) {
        margin-top: 2px;
    }
</style>
