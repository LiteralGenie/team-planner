<script lang="ts">
    import apIcon from '$lib/assets/tft/misc/statmodsabilitypowericon.png'
    import adIcon from '$lib/assets/tft/misc/statmodsattackdamageicon.png'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import RangeIcon from '$lib/icons/range-icon.svelte'
    import { last, sort } from 'radash'
    import type { CostTier, SlotFilter } from '../form-context/types'
    import { getActiveFilters } from '../form-context/utils'

    export let slot: SlotFilter

    $: activeFilters = getActiveFilters(slot)
    $: isEmpty = Object.entries(activeFilters ?? {}).length === 0

    $: showRangeOrDamage =
        activeFilters.range || activeFilters.damageType

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

<div class="w-16 text-sm py-1 text-muted-foreground flex flex-col">
    <h1>Filters:</h1>

    {#if isEmpty}
        <span>None</span>
    {/if}

    <div class="flex flex-col gap-[2px]">
        {#if activeFilters.cost}
            <span class="flex items-center gap-[3px]">
                <GoldIcon class="h-2 w-2" />
                {humanizeCostFilters(activeFilters.cost)}
            </span>
        {/if}
        {#if showRangeOrDamage}
            <div class="comma-group">
                {#if activeFilters.range}
                    <span>
                        <RangeIcon
                            activeClose={!!slot.byAttribute.range
                                .close}
                            activeMid={!!slot.byAttribute.range.mid}
                            activeLong={!!slot.byAttribute.range.long}
                            class="h-3 w-3 inline"
                        />
                    </span>
                {/if}

                {#if activeFilters.damageType?.includes('ad')}
                    <img src={adIcon} class="h-3 w-3 inline" />
                {/if}

                {#if activeFilters.damageType?.includes('ap')}
                    <img src={apIcon} class="h-3 w-3 inline" />
                {/if}
            </div>
        {/if}
    </div>
</div>

<style lang="postcss">
    .comma-group {
        line-height: 1;

        & > *:not(:last-child)::after {
            content: ' , ';
        }
    }
</style>
