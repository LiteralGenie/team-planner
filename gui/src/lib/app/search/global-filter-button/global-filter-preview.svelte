<script lang="ts" context="module">
</script>

<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import TraitIcon from '$lib/components/trait-icon.svelte'
    import {
        CHAMPIONS_BY_ID,
        CHAMPION_ICONS,
        TRAIT_ICONS
    } from '$lib/constants'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'

    const { form } = getFilterFormContext()

    $: bannedCostTiers = Object.entries($form.global.cost)
        .filter(([_, val]) => !val)
        .map(([cost, _]) => cost)
    $: bannedTraits = $form.global.traits
        .filter(({ included }) => !included)
        .map(({ id }) => id)
    $: bannedChampions = $form.global.champions
        .filter(({ included }) => !included)
        .map(({ id }) => id)

    $: emptyFilters =
        bannedCostTiers.length === 0 &&
        bannedTraits.length === 0 &&
        bannedChampions.length === 0
</script>

<div class="flex items-center justify-center gap-1">
    <h1 class="inline">Global Filters:</h1>

    {#if emptyFilters}
        <span>None</span>
    {:else if bannedCostTiers.length > 0}
        <div class="inline-flex gap-1 pl-1 items-center">
            <span class="relative">
                <div class="absolute h-full w-full slash"></div>
                <GoldIcon class="h-[0.75em] w-[0.75em]" />
            </span>
            <span class="comma-group">
                {#each bannedCostTiers as cost}
                    <span>{cost}</span>
                {/each}
            </span>
        </div>
    {/if}
    {#if bannedTraits.length > 0}
        <div class="inline-flex pl-1 gap-[0.0625em]">
            {#each bannedTraits as id}
                <div class="inline relative h-8 w-8">
                    <TraitIcon src={TRAIT_ICONS[id]} />

                    <div
                        class="absolute bottom-0 right-0 rounded-full bg-[#eb1a26] p-[2px]"
                    >
                        <XIcon
                            class="h-[0.5em] w-[0.5em] text-foreground"
                            stroke-width="3"
                        />
                    </div>
                </div>
            {/each}
        </div>
    {/if}
    {#if bannedChampions.length > 0}
        <div class="inline-flex pl-1 gap-[0.5em]">
            {#each bannedChampions as id}
                <div class="inline relative h-6 w-6">
                    <ChampionPortrait
                        cost={CHAMPIONS_BY_ID[id].tier}
                        src={CHAMPION_ICONS[id]}
                        hideInnerBorder={true}
                    />

                    <div
                        class="champion-mark absolute rounded-full bg-[#eb1a26] p-[2px]"
                    >
                        <XIcon
                            class="h-[0.5em] w-[0.5em] text-foreground"
                            stroke-width="3"
                        />
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style lang="postcss">
    .slash {
        background: linear-gradient(
            to top left,
            rgba(0, 0, 0, 0) 0%,
            rgba(0, 0, 0, 0) calc(50% - 1.1px),
            red 50%,
            rgba(0, 0, 0, 0) calc(50% + 1.1px),
            rgba(0, 0, 0, 0) 100%
        );
    }

    .comma-group {
        & > *:not(:last-child)::after {
            content: ', ';
        }
    }

    .champion-mark {
        bottom: -0.125em;
        right: -0.125em;
    }
</style>
