<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import { range } from 'radash'
    import type {
        AttributeSlotControls,
        AttributeSlotValues,
        CostTier
    } from '../../form-context/types'
    import type { CheckboxData } from '../checkbox-group/checkbox-group.svelte'
    import CheckboxGroup from '../checkbox-group/checkbox-group.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    const { form } = getFilterFormContext()

    $: options = (Array.from(range(1, 5)) as CostTier[]).map(
        (idx) => ({
            value: slotValues.cost[idx],
            label: idx.toString(),
            disabled: !$form.global.cost[idx],
            suffix: GoldIcon,
            onChange: (update: boolean) => {
                const ctrl = slotControls.controls.cost.controls[idx]
                ctrl.onChange(update)
            }
        })
    ) satisfies CheckboxData[]
</script>

<CheckboxGroup label="Cost" {options} />
