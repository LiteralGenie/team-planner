<script lang="ts">
    import type { AttributeSlot } from '../form-context/types'
    import CostInput from './cost-input.svelte'

    export let slot: AttributeSlot

    const costArray = slot.controls.cost
</script>

<fieldset class="flex gap-4">
    <legend class="pb-2">Tier</legend>

    <CostInput control={costArray.controls[1]} tier={1} />
    <CostInput control={costArray.controls[2]} tier={2} />
    <CostInput control={costArray.controls[3]} tier={3} />
    <CostInput control={costArray.controls[4]} tier={4} />
    <CostInput control={costArray.controls[5]} tier={5} />
</fieldset>
