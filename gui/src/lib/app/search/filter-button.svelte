<script lang="ts">
    import PencilIcon from '$lib/icons/pencil-icon.svelte'
    import PlusIcon from '$lib/icons/plus-icon.svelte'
    import UserGroupIcon from '$lib/icons/user-group-icon.svelte'
    import { objectify } from 'radash'

    export let variant: 'inactive' | 'active' | string = 'inactive'

    let championId: string | null = null
    $: {
        switch (variant) {
            case 'inactive':
                championId = null
                break
            case 'active':
                championId = null
                break
            default:
                championId = variant
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

<div class="hex hex-shadow p-1">
    <div class="hex outer-hex-color h-24 w-24 p-1">
        <div
            class="hex inner-hex-color h-full w-full flex justify-center items-center"
        >
            <button
                on:click
                type="button"
                class="h-full w-full relative text-5xl font-extralight flex justify-center items-center"
            >
                {#if championId}
                    <div
                        class="champion-image-hover-overlay absolute h-full w-full"
                    ></div>

                    <img src={getChampionIcon(championId)} />

                    <div class="pencil-icon absolute">
                        <PencilIcon class="h-8 w-8 font-bold" />
                    </div>
                {:else if variant === 'active'}
                    <UserGroupIcon class="h-11 w-11" />
                {:else}
                    <PlusIcon class="h-12 w-12" />
                {/if}
            </button>
        </div>
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

    .hex-shadow {
        filter: drop-shadow(5px 5px 5px hsl(var(--foreground) / 7%));
    }

    /* Show overlay on hover */
    .inner-hex-color,
    .champion-image-hover-overlay {
        transition: all 0.2s;

        &:hover {
            background-color: hsl(var(--background) / 40%);
        }
    }

    /* Show pencil icon on hover */
    .pencil-icon {
        visibility: hidden;
        pointer-events: none;
    }
    .inner-hex-color:hover .pencil-icon {
        visibility: visible;
    }
</style>
