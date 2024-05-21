<script lang="ts">
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'

    export let portal: string | null = null
    export let disabled = false
</script>

{#if !disabled}
    <Tooltip.Root
        disableHoverableContent={true}
        openDelay={100}
        closeOnPointerDown={true}
        {portal}
    >
        <Tooltip.Trigger
            class={disabled ? 'tooltip-trigger-disabled' : ''}
        >
            <slot />
        </Tooltip.Trigger>
        <Tooltip.Content class="boring-tooltip">
            <slot name="tooltip" />
        </Tooltip.Content>
    </Tooltip.Root>
{:else}
    <slot />
{/if}
