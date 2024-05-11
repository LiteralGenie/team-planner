<script lang="ts" context="module">
    import type { SvelteComponent } from 'svelte'

    export interface CheckboxData {
        value: boolean | Boolean
        label: string
        disabled?: boolean
        tooltip?: string
        suffix?: typeof SvelteComponent<any>
        suffixOpts?: any
        suffixClass?: string
        prefix?: typeof SvelteComponent<any>
        prefixOpts?: any
        prefixClass?: string
        onChange: (update: boolean) => void
    }
</script>

<script lang="ts">
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import CheckboxItem from './checkbox-item.svelte'

    export let options: CheckboxData[]
    export let label: string
    export let description: string = ''
</script>

<fieldset class="root flex flex-col">
    <legend class="">{label}</legend>

    {#if description}
        <p class="text-muted-foreground text-xs pt-1">
            {description}
        </p>
    {/if}

    <div class="h-[6px]"></div>

    <div class="flex gap-6 flex-wrap">
        {#each options as { onChange, tooltip, ...props }}
            {#if tooltip}
                <Tooltip.Root
                    group="spell"
                    openDelay={100}
                    closeOnPointerDown={true}
                    portal={'dialog'}
                >
                    <Tooltip.Trigger>
                        <CheckboxItem
                            on:change={(ev) => onChange(ev.detail)}
                            id={props.label}
                            {...props}
                        />
                    </Tooltip.Trigger>
                    <Tooltip.Content class="checkbox-tooltip-root">
                        <span>{tooltip}</span>
                    </Tooltip.Content>
                </Tooltip.Root>
            {:else}
                <CheckboxItem
                    on:change={(ev) => onChange(ev.detail)}
                    id={props.label}
                    {...props}
                />
            {/if}
        {/each}
    </div>
</fieldset>

<style lang="postcss">
    :global(.checkbox-tooltip-root) {
        @apply border-2;

        background-color: hsl(var(--popover) / var(--tw-bg-opacity));
        border-color: hsl(var(--border) / 75%);
    }
</style>
