<script lang="ts" context="module">
</script>

<script lang="ts">
    import {
        CHAMPIONS_BY_ID,
        CHAMPIONS_BY_TRAIT,
        CHAMPION_ICONS,
        TRAITS_BY_ID,
        TRAIT_ICONS
    } from '$lib/constants'
    import { interpolate_tooltip_images } from '$lib/utils/tooltips'
    import ChampionPortrait from './champion-portrait.svelte'

    export let trait_id: string

    $: trait = TRAITS_BY_ID[trait_id]
    $: iconSrc = TRAIT_ICONS[trait_id]
    $: html = interpolate_tooltip_images(trait.tooltip_html).replace(
        'placeholder="$ABILITY_ICON_SRC"',
        `src="${iconSrc}"`
    )
    $: others = CHAMPIONS_BY_TRAIT[trait_id]
</script>

<div class="root">
    <h1 class="text-base font-bold pb-1">{trait.display_name}</h1>

    {@html html}

    <div class="flex gap-2 border-t-2 pt-3 mt-2 pb-1">
        {#each others as c}
            <div class="h-8 w-8">
                <ChampionPortrait
                    src={CHAMPION_ICONS[c.character_id]}
                    cost={CHAMPIONS_BY_ID[c.character_id].tier}
                    hideInnerBorder={true}
                />
            </div>
        {/each}
    </div>
</div>

<style lang="postcss">
    .root :global(.trait-tooltip) {
        @apply text-sm;
    }

    .root :global(.conditional-effect + br) {
        display: none;
    }

    .root :global(.scaling-icon) {
        display: inline;
        height: 1em;
    }

    .root :global(span.conditional-effect:first-of-type) {
        @apply mt-2 pt-2 border-t-2;
    }
    .root :global(span.conditional-effect) {
        @apply block;

        color: hsl(var(--muted-foreground));
    }
</style>