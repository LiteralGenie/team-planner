<script lang="ts" context="module">
</script>

<script lang="ts">
    import { CHAMPIONS_BY_ID } from '$lib/constants'
    import { interpolate_tooltip_images } from '$lib/utils/tooltips'

    export let champion_id: string

    $: champion = CHAMPIONS_BY_ID[champion_id]
    $: html = interpolate_tooltip_images(champion.spell)
</script>

<div class="root">
    {@html html}
</div>

<style lang="postcss">
    .root :global(img) {
        display: inline;
    }

    .root :global(.tooltip-root) {
        display: grid;
        grid-template-areas:
            'title title'
            'desc  desc'
            'left  right';
        gap: 1em;
    }

    .root :global(.spell-name) {
        grid-area: title;
    }

    .root :global(.spell-description) {
        grid-area: desc;
    }

    .root :global(.post-script-left) {
        grid-area: left;
    }

    .root :global(.post-script-right) {
        grid-area: right;
    }
</style>
