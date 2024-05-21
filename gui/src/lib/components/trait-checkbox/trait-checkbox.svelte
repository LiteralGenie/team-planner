<script lang="ts" context="module">
    export type TraitCheckboxValue = 'included' | 'excluded' | null
</script>

<script lang="ts">
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import TraitTooltip from '../trait-tooltip.svelte'
    import TraitCheckboxButton from './trait-checkbox-button.svelte'
    import TraitCheckboxLabel from './trait-checkbox-label.svelte'

    export let id: string
    export let value: TraitCheckboxValue
    export let disabled = false
    export let disabledValue: TraitCheckboxValue = 'excluded'
    export let disabledTooltip: string = ''
    export let portal: string | undefined = undefined
</script>

<div class:disabled class="root">
    <Tooltip.Root
        openDelay={100}
        closeOnPointerDown={true}
        {portal}
        group="trait"
        disableHoverableContent={true}
    >
        <Tooltip.Trigger>
            <div class="flex flex-col">
                <TraitCheckboxButton
                    on:click
                    {id}
                    {value}
                    {disabled}
                    {disabledValue}
                />

                <TraitCheckboxLabel
                    {id}
                    {value}
                    {disabled}
                    {disabledValue}
                />
            </div>
        </Tooltip.Trigger>
        <Tooltip.Content class="boring-tooltip max-w-[400px]">
            {#if disabled}
                {disabledTooltip}
            {:else}
                <TraitTooltip trait_id={id} />
            {/if}
        </Tooltip.Content>
    </Tooltip.Root>
</div>

<style lang="postcss">
    .root.disabled > :global(button) {
        cursor: default;
    }
</style>
