<script lang="ts">
    import type { CostTier } from '$lib/app/form-context/types'
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'

    export let src: string
    export let label: string
    export let cost: CostTier

    export let state: 'included' | 'excluded' | null
</script>

<div
    class="root flex flex-col justify-center items-center text-center gap-[1px]"
    class:active={state !== null}
>
    <button
        on:click
        type="button"
        class="h-12 w-12 relative select-none"
    >
        <!-- @todo description tooltip -->
        <ChampionPortrait {src} {cost} />

        <!-- Selection indicator -->
        {#if state === 'included' || state === 'excluded'}
            <div
                class:green={state === 'included'}
                class:red={state === 'excluded'}
                class="mark absolute bottom-[2px] right-[2px] rounded-full p-[2px] text-foreground"
            >
                {#if state === 'included'}
                    <CheckmarkIcon class="h-[0.9em] w-[0.9em]" />
                {:else if state === 'excluded'}
                    <XIcon class="h-[0.9em] w-[0.9em]" />
                {/if}
            </div>
        {/if}
    </button>

    <!-- Label -->
    <span class="text-xs text-muted-foreground whitespace-nowrap">
        {label}
    </span>
</div>

<style lang="postcss">
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
    }
    .root:not(.active) button {
        opacity: 0.5;

        &:hover {
            opacity: 0.75;
        }
    }
    .active span {
        @apply text-foreground;
    }

    /* Prevent text wrap from creating uneven row heights */
    span {
        overflow: hidden;
        white-space: nowrap;
    }
</style>
