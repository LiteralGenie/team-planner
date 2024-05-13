<script lang="ts" context="module">
</script>

<script lang="ts">
    import {
        CHAMPIONS_BY_TRAIT,
        TRAITS_BY_ID,
        TRAIT_ICONS
    } from '$lib/constants'
    import { interpolate_tooltip_images } from '$lib/utils/tooltips'
    import ChampionPortrait from './champion-portrait.svelte'

    export let trait_id: string
    export let traitLevelIdx: number | null = null

    $: trait = TRAITS_BY_ID[trait_id]
    $: iconSrc = TRAIT_ICONS[trait_id]
    $: html = interpolate_tooltip_images(trait.tooltip_html).replace(
        'placeholder="$ABILITY_ICON_SRC"',
        `src="${iconSrc}"`
    )
    $: others = CHAMPIONS_BY_TRAIT[trait_id]

    let rootEl: HTMLElement

    $: {
        if (rootEl && traitLevelIdx !== null) {
            const rowEls = [
                ...rootEl.querySelectorAll('.conditional-effect')
            ]
            rowEls[traitLevelIdx]?.classList.add('active')
        }
    }
</script>

<div bind:this={rootEl} class="root">
    <h1 class="text-base font-bold pb-1">{trait.display_name}</h1>

    {@html html}

    <div class="flex gap-2 border-t-2 pt-3 mt-2 pb-1">
        {#each others as c}
            <div class="h-8 w-8">
                <ChampionPortrait
                    id={c.character_id}
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
    .root :global(.conditional-effect.active) {
        color: hsl(var(--foreground));
    }
</style>
