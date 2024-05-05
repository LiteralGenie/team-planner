<script lang="ts" context="module">
    import TRAITS from '$lib/assets/tft/tfttraits.json'
    import { objectify } from 'radash'

    const files = import.meta.glob('$lib/assets/tft/traits/*.png', {
        eager: true
    })
    const srcs: Record<string, string> = objectify(
        Array.from(Object.keys(files)),
        (path: string) => {
            path = path.split('/').slice(-1)[0]
            path = path.split('.')[0]
            return path
        },
        (path) => path
    )

    const TRAITS_BY_ID = objectify(
        TRAITS,
        (t) => t.trait_id,
        (t) => t
    )
</script>

<script lang="ts">
    import type {
        CostTier,
        SlotFilter
    } from '$lib/app/form-context/types'
    import { getActiveFilters } from '$lib/app/form-context/utils'
    import apIcon from '$lib/assets/tft/misc/statmodsabilitypowericon.png'
    import adIcon from '$lib/assets/tft/misc/statmodsattackdamageicon.png'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import RangeIcon from '$lib/icons/range-icon.svelte'
    import { last, sort } from 'radash'
    import TraitFilterPreview from './trait-filter-preview.svelte'

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

<div
    class="min-w-20 text-sm py-1 text-muted-foreground flex flex-col"
>
    <h1>Filters:</h1>

    {#if isEmpty}
        <span>None</span>
    {/if}

    <div class="flex flex-col gap-[3px]">
        {#if activeFilters.cost}
            <span class="flex items-center gap-[3px]">
                <GoldIcon class="h-2 w-2" />
                {humanizeCostFilters(activeFilters.cost)}
            </span>
        {/if}
        {#if showRangeOrDamage}
            <div class="comma-group">
                {#if activeFilters.range?.includes('close')}
                    <span>
                        <RangeIcon
                            activeClose={!!slot.byAttribute.range
                                .close}
                            class="h-3 w-3 inline"
                        />
                    </span>
                {/if}

                {#if activeFilters.range?.includes('mid')}
                    <span>
                        <RangeIcon
                            activeMid={!!slot.byAttribute.range.mid}
                            class="h-3 w-3 inline"
                        />
                    </span>
                {/if}

                {#if activeFilters.range?.includes('long')}
                    <span>
                        <RangeIcon
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
        {#if activeFilters.traits}
            <span>
                {#each activeFilters.traits as { id, state }}
                    <TraitFilterPreview
                        trait_id={id}
                        checked={state === 1}
                    />
                {/each}
            </span>
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
