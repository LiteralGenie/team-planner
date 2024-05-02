<script lang="ts" context="module">
    type Cost = 1 | 2 | 3 | 4 | 5
</script>

<script lang="ts">
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import { range } from 'radash'
    import type {
        AttributeSlotControls,
        AttributeSlotValues
    } from '../form-context/types'
    import type { CheckboxData } from './checkbox-group/checkbox-group.svelte'
    import CheckboxGroup from './checkbox-group/checkbox-group.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    $: options = (Array.from(range(1, 5)) as Cost[]).map((idx) => ({
        value: slotValues.cost[idx],
        label: idx.toString(),
        suffix: GoldIcon,
        onChange: (update: boolean) => {
            const ctrl = slotControls.controls.cost.controls[idx]
            ctrl.onChange(update)
        }
    })) satisfies CheckboxData[]
</script>

<CheckboxGroup label="Cost" {options} />
