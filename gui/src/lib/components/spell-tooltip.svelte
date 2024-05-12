<script lang="ts" context="module">
</script>

<script lang="ts">
    import { mapRangeValueToType } from '$lib/app/form-context/utils'

    import {
        ABILITY_ICONS,
        AD_ICON,
        AP_ICON,
        CHAMPIONS_BY_ID,
        MANA_ICON
    } from '$lib/constants'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import RangeIcon from '$lib/icons/range-icon.svelte'
    import { interpolate_tooltip_images } from '$lib/utils/tooltips'
    import TraitIcon from './trait-icon.svelte'

    export let champion_id: string

    $: champion = CHAMPIONS_BY_ID[champion_id]
    $: iconSrc = ABILITY_ICONS[champion_id]
    $: html = interpolate_tooltip_images(champion.spell).replace(
        'placeholder="$ABILITY_ICON_SRC"',
        `src="${iconSrc}"`
    )

    $: rangeType = mapRangeValueToType(champion.stats.range)
</script>

<div class="root flex flex-col">
    {@html html}

    <div
        class="border-t-2 pt-2 mt-2 text-sm text-muted-foreground flex justify-between items-center"
    >
        <span>Traits:</span>

        <span class="comma-group flex gap-1">
            {#each champion.traits as t}
                <div class="flex items-center">
                    <span class="trait-label">{t.name}</span>

                    <div
                        class="trait-icon inline-block h-[1.75em] w-[1.75em]"
                    >
                        <TraitIcon id={t.id} variant="sm" />
                    </div>
                </div>
            {/each}
        </span>
    </div>

    <div
        class="pt-1 text-sm text-muted-foreground flex items-center justify-between"
    >
        <span>Stats:</span>

        <div class="flex justify-end gap-3">
            <span class="flex gap-1 items-center">
                {champion.tier}
                <GoldIcon class="h-[1em] ml-[-0.125em]" />
            </span>

            <span class="flex gap-1 items-center">
                {Math.floor(champion.stats.damage)}
                <img class="h-[1em]" src={AD_ICON} />
            </span>

            <span class="flex gap-1 items-center">
                100
                <img class="h-[1em]" src={AP_ICON} />
            </span>

            <span class="flex gap-[0.0625em] items-center">
                {Math.floor(champion.stats.mana)}
                <img class="h-[1em]" src={MANA_ICON} />
            </span>

            <span class="flex gap-1 items-center">
                {Math.floor(champion.stats.range)}
                <RangeIcon
                    class="h-[1em]"
                    activeClose={rangeType === 'close'}
                    activeMid={rangeType === 'mid'}
                    activeLong={rangeType === 'long'}
                />
            </span>
        </div>
    </div>
</div>

<style lang="postcss">
    .root :global(.scaling-icon) {
        display: inline;
    }

    .root :global(.tooltip-root) {
        display: grid;
        grid-template-areas:
            'icon  title title title title title'
            'desc  desc  desc  desc  desc  desc '
            'deets deets deets deets deets deets';
        grid-template-columns: max-content 1fr;
        gap: 1em;

        user-select: text !important;

        @apply text-sm;
    }

    .root :global(.spell-icon) {
        grid-area: icon;

        height: 100%;
        width: 100%;
        max-height: 100px;
        max-width: 100px;
    }

    .root :global(.spell-name) {
        grid-area: title;

        @apply text-base;
    }

    .root :global(.spell-description) {
        grid-area: desc;

        border-top: 2px solid hsl(var(--border));
        border-bottom: 2px solid hsl(var(--border));

        padding-top: 0.5em;
        padding-bottom: 0.5em;
    }

    .root :global(.post-script) {
        grid-area: deets;

        @apply flex flex-col;
    }

    .root :global(.post-script) {
        display: grid;
        grid-template-columns: 1fr max-content;
        gap: 0 1em;
    }

    .root :global(.post-script > :nth-child(2n)) {
        text-align: end;
    }

    .root :global(.rules) {
        @apply text-muted-foreground text-xs pt-2 italic;
    }

    .root :global(.scaling-icon) {
        @apply h-4 w-4 mx-[0.0625em];
    }

    .comma-group {
        & > *:not(:last-child)::after {
            padding-left: 0.125em;
            content: ', ';
        }
    }

    .trait-label {
        @apply mr-1;
    }
    .trait-icon:not(:last-child) {
        @apply mr-1;
    }
</style>
