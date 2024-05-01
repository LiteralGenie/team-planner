<script lang="ts">
    import { type FilterForm } from '../form-context/context'
    import type { FormControlsContainer } from '../form-context/utils'
    import CostInput from './cost-input.svelte'

    export let slot: FormControlsContainer<
        FilterForm['slots'][number]
    >

    const byAttribute = slot.controls.byAttribute
    const costArray = byAttribute.controls.cost
</script>

<fieldset class="flex gap-4">
    <legend class="pb-2">Tier</legend>

    <CostInput control={costArray.controls[0]} tier={0} />
    <CostInput control={costArray.controls[1]} tier={1} />
    <CostInput control={costArray.controls[2]} tier={2} />
    <CostInput control={costArray.controls[3]} tier={3} />
    <CostInput control={costArray.controls[4]} tier={4} />
</fieldset>
