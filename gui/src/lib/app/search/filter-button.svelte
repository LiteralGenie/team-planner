<script lang="ts">
    import PencilIcon from '$lib/icons/pencil-icon.svelte'
    import PlusIcon from '$lib/icons/plus-icon.svelte'
    import UserIcon from '$lib/icons/user-icon.svelte'
    import { objectify } from 'radash'
    import type { ComponentType } from 'svelte'

    export let variant: 'inactive' | 'active' | string = 'inactive'

    let icon: ComponentType | string
    $: {
        switch (variant) {
            case 'inactive':
                icon = PlusIcon
                break
            case 'active':
                icon = UserIcon
                break
            default:
                icon = variant
        }
    }

    let icons = import.meta.glob('$lib/assets/tft/icons/*.png', {
        eager: true
    })
    let champion_icons = objectify(
        Object.keys(icons),
        (path: string) =>
            path.split('/').slice(-1)[0].replace('.png', ''),
        (path: string) => path
    )

    function getChampionIcon(id: string) {
        return champion_icons[id] as string
    }
</script>

<div class="hex outer-hex-color h-24 w-24 p-1">
    <div
        class="hex inner-hex-color h-full w-full flex justify-center items-center"
    >
        <button
            on:click
            type="button"
            class="h-full w-full relative text-5xl font-extralight flex justify-center items-center"
        >
            {#if typeof icon === 'string'}
                <div
                    class="champion-image-hover-overlay absolute h-full w-full"
                ></div>

                <img src={getChampionIcon(variant)} />

                <div class="pencil-icon absolute">
                    <PencilIcon class="h-8 w-8 font-bold" />
                </div>
            {:else}
                <svelte:component this={icon} class="h-12 w-12" />
            {/if}
        </button>
    </div>
</div>

<style lang="postcss">
    .outer-hex-color {
        background-color: hsl(var(--muted-foreground) / 30%);
    }

    .inner-hex-color {
        background-color: hsl(var(--background) / 90%);
        color: hsl(var(--muted-foreground));
    }

    /* Show overlay on hover */
    .inner-hex-color:hover,
    .champion-image-hover-overlay:hover {
        background-color: hsl(var(--background) / 40%);
    }

    /* Show pencil icon on hover */
    .pencil-icon {
        visibility: hidden;
        pointer-events: none;
    }
    .inner-hex-color:hover .pencil-icon {
        visibility: visible;
    }

    .hex {
        /**
         * Draws hex in counter-clockwise fashion, starting from bottom corners
         *    ________
         *   /        \
         *  /          \
         * /            \
         * \            /
         *  \          /
         *   \________/
         *   x1      x2
         *
         * A regular hexagon has equal interior angles which works out to 120deg each
         * and assuming a side length of 1, that works out to
         * x1 = 0.5 or 25% of the total width (=2)
         * so x2 is (0.5 + 1) which is 75% of the total width
         */
        clip-path: polygon(
            25% 0%,
            75% 0%,
            100% 50%,
            75% 100%,
            25% 100%,
            0% 50%
        );
    }
</style>
