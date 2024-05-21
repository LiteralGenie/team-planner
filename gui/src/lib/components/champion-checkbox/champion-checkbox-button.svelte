<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'
    import type { ChampionCheckboxValue } from './champion-checkbox.svelte'

    export let id: string
    export let value: ChampionCheckboxValue
    export let disabled = false
    export let disabledValue: ChampionCheckboxValue = 'excluded'

    $: actualValue = disabled ? disabledValue : value
</script>

<button
    class:active={actualValue !== null}
    {disabled}
    on:click
    type="button"
    class="h-12 w-12 relative select-none"
>
    <ChampionPortrait {id} />

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
    button:not(.active),
    button[disabled] {
        opacity: 0.4;

        &:hover {
            opacity: 0.75;
        }
    }
</style>
