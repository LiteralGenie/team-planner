<script lang="ts">
    import type { CostTier } from '$lib/app/form-context/types'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import { range } from 'radash'
    import { getFilterFormContext } from '../../form-context/context'
    import CheckboxGroup, {
        type CheckboxData
    } from '../checkbox-group/checkbox-group.svelte'

    const { form, controls } = getFilterFormContext()

    const options = (Array.from(range(1, 5)) as CostTier[]).map(
        (idx) => ({
            value: true,
            label: idx.toString(),
            suffix: GoldIcon,
            onChange: (update: boolean) => {}
        })
    ) satisfies CheckboxData[]
</script>

<form class="h-full w-full pt-4 px-6">
    <h1 class="text-xl text-foreground font-bold mb-4">
        Global Filters
    </h1>

    <div>
        <CheckboxGroup label="Cost" {options} />
    </div>
</form>

<style lang="postcss">
</style>
