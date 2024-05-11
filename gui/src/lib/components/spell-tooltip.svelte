<script lang="ts" context="module">
</script>

<script lang="ts">
    import { ABILITY_ICONS, CHAMPIONS_BY_ID } from '$lib/constants'
    import { interpolate_tooltip_images } from '$lib/utils/tooltips'

    export let champion_id: string

    $: champion = CHAMPIONS_BY_ID[champion_id]
    $: iconSrc = ABILITY_ICONS[champion_id]
    $: html = interpolate_tooltip_images(champion.spell).replace(
        '$ABILITY_ICON_SRC',
        iconSrc
    )
</script>

<div class="root">
    {@html html}
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
</style>
