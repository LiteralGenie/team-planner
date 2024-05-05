<script lang="ts" context="module">
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
</script>

<script lang="ts">
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'

    export let trait_id: string
    export let checked: boolean

    $: src = srcs[trait_id]
</script>

<span
    class="root inline-flex flex-col justify-center items-center text-center gap-[1px]"
>
    <button
        on:click
        type="button"
        class="h-[28px] w-[28px] relative select-none"
    >
        <!-- Hex icon -->
        <div class="hex hover-fill p-[1px]">
            <div class="hex dark-fill p-[1px]">
                <div class="hex light-fill p-[1px]">
                    <div class="hex dark-fill">
                        <img class="h-[66%] w-[66%]" {src} />
                    </div>
                </div>
            </div>
        </div>

        <!-- Selection indicator -->
        <div
            class:green={checked}
            class:red={!checked}
            class="mark absolute bottom-[2px] right-[3px] rounded-full p-[2px] text-foreground"
        >
            {#if checked}
                <CheckmarkIcon class="h-[6px] w-[6px]" />
            {:else}
                <XIcon class="h-[6px] w-[6px]" />
            {/if}
        </div>
    </button>
</span>

<style lang="postcss">
    .hex {
        @apply h-full w-full flex justify-center items-center;

        &.dark-fill {
            background-color: #232323;
        }

        &.light-fill {
            background-color: #6b6d6b;
        }
    }

    .mark {
        /* Prevent hover effect from disappearing on icon hover */
        pointer-events: none;

        &.green {
            background-color: #39b549;
        }
        &.red {
            background-color: #eb1a26;
        }
    }

    /* Highlight on hover / selection */
    button {
        transition: all 0.2s;

        &:hover .hover-fill {
            filter: drop-shadow(0px 0px 20px rgba(255, 199, 46, 0.9));
        }

        @apply text-foreground;
    }
</style>
