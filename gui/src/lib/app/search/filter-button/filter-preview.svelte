<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type {
        CostTier,
        SlotFilter
    } from '$lib/app/form-context/types'
    import { getActiveSlotFilters } from '$lib/app/form-context/utils'
    import apIcon from '$lib/assets/tft/misc/statmodsabilitypowericon.png'
    import adIcon from '$lib/assets/tft/misc/statmodsattackdamageicon.png'
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import SpellTooltip from '$lib/components/spell-tooltip.svelte'
    import TraitTooltip from '$lib/components/trait-tooltip.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import { CHAMPIONS_BY_ID, CHAMPION_ICONS } from '$lib/constants'
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import RangeIcon from '$lib/icons/range-icon.svelte'
    import { last, sort } from 'radash'
    import TraitFilterPreview from './trait-filter-preview.svelte'

    export let slot: SlotFilter

    const { form } = getFilterFormContext()

    $: activeFilters = getActiveSlotFilters($form.global, slot)
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

<div class="min-w-20 py-1 flex flex-col">
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
                {#each activeFilters.traits as { id, included }}
                    <Tooltip.Root
                        group="preview"
                        openDelay={500}
                        closeOnPointerDown={true}
                    >
                        <Tooltip.Trigger class="cursor-default">
                            <TraitFilterPreview
                                trait_id={id}
                                checked={included}
                            />
                        </Tooltip.Trigger>
                        <Tooltip.Content
                            class="spell-tooltip-container"
                        >
                            <TraitTooltip trait_id={id} />
                        </Tooltip.Content>
                    </Tooltip.Root>
                {/each}
            </span>
        {/if}
        {#if activeFilters.champions}
            <div class="champion-grid pt-1">
                {#each activeFilters.champions as { id }}
                    <Tooltip.Root
                        group="preview"
                        openDelay={500}
                        closeOnPointerDown={true}
                    >
                        <Tooltip.Trigger class="cursor-default">
                            <div class="relative">
                                <ChampionPortrait
                                    cost={CHAMPIONS_BY_ID[id].tier}
                                    src={CHAMPION_ICONS[id]}
                                    hideInnerBorder={true}
                                />

                                <div
                                    class="mark absolute bottom-[-0px] right-[-0px] rounded-full p-[2px] text-foreground"
                                >
                                    <CheckmarkIcon
                                        class="h-[6px] w-[6px]"
                                    />
                                </div>
                            </div>
                        </Tooltip.Trigger>
                        <Tooltip.Content
                            class="spell-tooltip-container"
                        >
                            <SpellTooltip champion_id={id} />
                        </Tooltip.Content>
                    </Tooltip.Root>
                {/each}
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

    .champion-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, 1.75em);
        align-items: start;
        gap: 6px 6px;
    }

    .mark {
        background-color: var(--checked-color);
    }
</style>
