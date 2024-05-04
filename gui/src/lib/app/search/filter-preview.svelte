<script lang="ts" context="module">
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import { last, sort } from 'radash'
    import type {
        CostTier,
        RangeType,
        SlotFilter
    } from '../form-context/types'

    interface ActiveFilters {
        cost?: CostTier[]
        range?: RangeType[]
    }
</script>

<script lang="ts">
    import RangeIcon from '$lib/icons/range-icon.svelte'
    import { filterMap, someFalse } from '$lib/utils/misc'

    export let slot: SlotFilter

    $: activeFilters = getActiveFilters(slot)
    $: isEmpty = Object.entries(activeFilters ?? {}).length === 0

    function getActiveFilters(slot: SlotFilter): ActiveFilters {
        const attrFilters = slot.byAttribute

        let activeFilters: ActiveFilters = {}

        if (someFalse(attrFilters.cost)) {
            activeFilters.cost = filterMap<CostTier, any>(
                Object.entries(attrFilters.cost),
                ([cost, val]) => (val ? cost : null)
            )
        }

        if (someFalse(attrFilters.range)) {
            activeFilters.range = filterMap<RangeType, any>(
                Object.entries(attrFilters.range),
                ([range, val]) => (val ? range : null)
            )
        }

        console.log(activeFilters)
        return activeFilters
    }

    function humanizeCostFilters(costs: CostTier[]): string {
        let bin: CostTier[] = []
        let ranges: CostTier[][] = []

        costs = sort(costs, (c) => c)
        bin.push(costs[0])

        for (let c of costs.slice(1)) {
            const diff = c - last(bin)!
            if (diff > 1) {
                ranges.push(bin)
                bin = [c]
            } else {
                bin.push(c)
            }
        }

        ranges.push(bin)

        const rangeStrings = ranges.map((rng) => {
            if (rng.length === 1) {
                return rng[0].toString()
            } else {
                return `${rng[0]}-${last(rng)}`
            }
        })

        return rangeStrings.join(', ')
    }
</script>

<div
    class="text-xs py-1 text-muted-foreground flex flex-col gap-[2px]"
>
    <h1 class:invisible={isEmpty}>Filters:</h1>

    <div class="flex flex-col text-[11px]">
        {#if activeFilters.cost}
            <span class="flex items-center gap-[3px]">
                <GoldIcon variant="muted" class="h-2 w-2" />
                {humanizeCostFilters(activeFilters.cost)}
            </span>
        {/if}
        {#if activeFilters.range}
            <span class="flex items-center gap-[3px]">
                <RangeIcon
                    activeClose={!!slot.byAttribute.range.close}
                    activeMid={!!slot.byAttribute.range.mid}
                    activeLong={!!slot.byAttribute.range.long}
                    class="h-4 w-4"
                />
            </span>
        {/if}
    </div>
</div>

<style lang="postcss">
</style>
