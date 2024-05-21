<script lang="ts">
    import { TRAIT_ICONS } from '$lib/constants'
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'
    import type { TraitCheckboxValue } from './trait-checkbox.svelte'

    export let id: string
    export let value: TraitCheckboxValue
    export let disabled = false
    export let disabledValue: TraitCheckboxValue = 'excluded'

    $: src = TRAIT_ICONS[id]
    $: actualValue = disabled ? disabledValue : value
</script>

<button
    {disabled}
    on:click
    type="button"
    class:active={actualValue !== null}
    class="h-12 w-12 relative select-none"
>
    <!-- Hex icon -->
    <div class="hex hover-fill p-[2px]">
        <div class="hex dark-fill p-[2px]">
            <div class="hex light-fill p-[2px]">
                <div class="hex dark-fill">
                    <img class="h-[66%] w-[66%]" {src} />
                </div>
            </div>
        </div>
    </div>

    <!-- Selection indicator -->
    {#if actualValue === 'included' || actualValue === 'excluded'}
        <div
            class:green={actualValue === 'included'}
            class:red={actualValue === 'excluded'}
            class="mark absolute bottom-[2px] right-[3px] rounded-full p-[2px] text-foreground"
        >
            {#if actualValue === 'included'}
                <CheckmarkIcon class="h-3 w-3" />
            {:else if actualValue === 'excluded'}
                <XIcon class="h-3 w-3" />
            {/if}
        </div>
    {/if}
</button>

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
            background-color: var(--checked-color);
        }
        &.red {
            background-color: var(--unchecked-color);
        }
    }

    /* Highlight on hover / selection */
    button {
        transition: all 0.2s;

        &:hover .hover-fill,
        &:focus .hover-fill {
            filter: drop-shadow(0px 0px 20px rgba(255, 199, 46, 0.9));
        }
    }
    button:not(.active),
    button[disabled] {
        opacity: 0.3;

        &:hover {
            opacity: 0.5;
        }
    }
</style>
