<script lang="ts" context="module">
    type Cost = 1 | 2 | 3 | 4 | 5
</script>

<script lang="ts">
    import { range } from 'radash'
    import type {
        AttributeSlotControls,
        AttributeSlotValues
    } from '../form-context/types'
    import CostInput from './cost-input.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    const costs = Array.from(range(1, 5)) as Cost[]

    const costArray = slotControls.controls.cost

    function handleChange(idx: Cost, ev: CustomEvent<boolean>) {
        const ctrl = slotControls.controls.cost.controls[idx]
        ctrl.onChange(ev.detail)
    }
</script>

<fieldset class="flex gap-4">
    <legend class="pb-2">Cost</legend>

    <div class="flex gap-6">
        {#each costs as idx}
            <CostInput
                cost={idx}
                value={!!slotValues.cost[idx]}
                on:change={(ev) => handleChange(idx, ev)}
            />
        {/each}
    </div>
</fieldset>
