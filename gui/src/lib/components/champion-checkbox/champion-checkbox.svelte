<script lang="ts" context="module">
    export type ChampionCheckboxValue = 'included' | 'excluded' | null
</script>

<script lang="ts">
    import * as Popover from '$lib/components/ui/popover/index.js'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import SpellTooltip from '../spell-tooltip.svelte'
    import Button from '../ui/button/button.svelte'
    import ChampionCheckboxButton from './champion-checkbox-button.svelte'
    import ChampionCheckboxLabel from './champion-checkbox-label.svelte'

    export let id: string

    export let value: ChampionCheckboxValue
    export let disabled = false
    export let disabledValue: ChampionCheckboxValue = 'excluded'
    export let disabledTooltip: string = ''
    export let portal: string | undefined = undefined

    $: actualValue = disabled ? disabledValue : value
</script>

<!-- Desktop version -->
<div
    class:disabled
    class="root hidden md:flex flex-col justify-center items-center text-center gap-[1px]"
    class:active={actualValue !== null}
>
    <Tooltip.Root
        group="spell"
        openDelay={500}
        closeOnPointerDown={true}
        {portal}
        disableHoverableContent={true}
    >
        <Tooltip.Trigger
            class="cursor-default flex flex-col gap-1 items-center justify-center"
        >
            <ChampionCheckboxButton
                on:click
                {id}
                {value}
                {disabled}
                {disabledValue}
            />

            <ChampionCheckboxLabel
                {id}
                {value}
                {disabled}
                {disabledValue}
            />
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

<!-- Mobile version -->
<div
    class:disabled
    class="root md:hidden flex flex-col justify-center items-center text-center"
    class:active={actualValue !== null}
>
    <ChampionCheckboxButton
        on:click
        {id}
        {value}
        {disabled}
        {disabledValue}
    />

    <Popover.Root portal={'dialog'}>
        <Popover.Trigger asChild let:builder>
            <Button
                builders={[builder]}
                variant="link"
                class="h-6 text-inherit"
            >
                <ChampionCheckboxLabel
                    {id}
                    {value}
                    {disabled}
                    {disabledValue}
                />
            </Button>
        </Popover.Trigger>
        <Popover.Content class="spell-tooltip-container">
            {#if disabled}
                {disabledTooltip}
            {:else}
                <SpellTooltip champion_id={id} />
            {/if}
        </Popover.Content>
    </Popover.Root>
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
