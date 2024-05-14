<script lang="ts" context="module">
    export type ChampionCheckboxValue = 'included' | 'excluded' | null
</script>

<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import { CHAMPIONS_BY_ID } from '$lib/constants'
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'
    import SpellTooltip from './spell-tooltip.svelte'

    export let id: string

    $: champion = CHAMPIONS_BY_ID[id]
    $: label = champion.display_name

    export let value: ChampionCheckboxValue
    export let disabled = false
    export let disabledValue: ChampionCheckboxValue = 'excluded'
    export let disabledTooltip: string = ''

    $: actualValue = disabled ? disabledValue : value
</script>

<div
    class:disabled
    class="root flex flex-col justify-center items-center text-center gap-[1px]"
    class:active={actualValue !== null}
>
    <Tooltip.Root
        group="spell"
        openDelay={1000}
        closeOnPointerDown={true}
        portal={'dialog'}
        disableHoverableContent={true}
    >
        <Tooltip.Trigger
            class="cursor-default flex flex-col gap-1 items-center justify-center"
        >
            <button
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
                            <CheckmarkIcon
                                class="h-[0.9em] w-[0.9em]"
                            />
                        {:else if actualValue === 'excluded'}
                            <XIcon class="h-[0.9em] w-[0.9em]" />
                        {/if}
                    </div>
                {/if}
            </button>

            <!-- Label -->
            <span
                class="text-xs text-muted-foreground whitespace-nowrap"
            >
                {label}
            </span>
        </Tooltip.Trigger>
        <Tooltip.Content
            class={disabled
                ? 'boring-tooltip'
                : 'spell-tooltip-container'}
        >
            {#if disabled}
                {disabledTooltip}
            {:else}
                <SpellTooltip champion_id={id} />
            {/if}
        </Tooltip.Content>
    </Tooltip.Root>
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
        opacity: 0.4;

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
