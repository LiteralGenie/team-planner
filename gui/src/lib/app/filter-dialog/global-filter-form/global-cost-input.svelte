<script lang="ts">
    import type { CostTier } from '$lib/app/form-context/types'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import { range } from 'radash'
    import { getFilterFormContext } from '../../form-context/context'
    import CheckboxGroup, {
        type CheckboxData
    } from '../checkbox-group/checkbox-group.svelte'

    const { form, controls } = getFilterFormContext()

    $: options = (Array.from(range(1, 5)) as CostTier[]).map(
        (idx) => ({
            value: $form.global.cost[idx],
            label: idx.toString(),
            suffix: GoldIcon,
            onChange: (update: boolean) => {
                controls.global.controls.cost.controls[idx].onChange(
                    update
                )
            }
        })
    ) satisfies CheckboxData[]
</script>

<CheckboxGroup
    label="Cost"
    description="Deselect a cost tier to disable it for all slots."
    {options}
/>

<style lang="postcss">
</style>
