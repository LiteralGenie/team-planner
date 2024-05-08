<script lang="ts" context="module">
    export type ChampionCheckboxValue = 'included' | 'excluded' | null
</script>

<script lang="ts">
    import type { CostTier } from '$lib/app/form-context/types'
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'

    export let src: string
    export let label: string
    export let cost: CostTier

    export let value: ChampionCheckboxValue
    export let disabled = false
    export let disabledValue: ChampionCheckboxValue = 'excluded'

    $: actualValue = disabled ? disabledValue : value
</script>

<div
    class:disabled
    class="root flex flex-col justify-center items-center text-center gap-[1px]"
    class:active={actualValue !== null}
>
    <button
        {disabled}
        on:click
        type="button"
        class="h-12 w-12 relative select-none"
    >
        <!-- @todo description tooltip -->
        <ChampionPortrait {src} {cost} />

        <!-- Selection indicator -->
        {#if actualValue === 'included' || actualValue === 'excluded'}
            <div
                class:green={actualValue === 'included'}
                class:red={actualValue === 'excluded'}
                class="mark absolute bottom-[2px] right-[2px] rounded-full p-[2px] text-foreground"
            >
                {#if actualValue === 'included'}
                    <CheckmarkIcon class="h-[0.9em] w-[0.9em]" />
                {:else if actualValue === 'excluded'}
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
            background-color: var(--checked-color);
        }
        &.red {
            background-color: var(--unchecked-color);
        }
    }

    /* Highlight on hover / selection */
    button {
        transition: all 0.2s;
    }
    .root:not(.active) button,
    button[disabled] {
        opacity: 0.5;

        &:hover {
            opacity: 0.75;
        }
    }
    root:not(.disabled) .active span {
        @apply text-foreground;
    }

    /* Prevent text wrap from creating uneven row heights */
    span {
        overflow: hidden;
        white-space: nowrap;
    }
</style>
