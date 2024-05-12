<script lang="ts">
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'

    export let tooltip: string = ''
    export let portal: string | null = null
    export let disabled = false
</script>

{#if tooltip}
    <Tooltip.Root openDelay={100} closeOnPointerDown={true} {portal}>
        <Tooltip.Trigger
            class={disabled ? 'tooltip-trigger-disabled' : ''}
        >
            <slot />
        </Tooltip.Trigger>
        <Tooltip.Content class="boring-tooltip">
            <span>{tooltip}</span>
        </Tooltip.Content>
    </Tooltip.Root>
{:else}
    <slot />
{/if}
