<script lang="ts" context="module">
    import type { SvelteComponent } from 'svelte'

    export interface CheckboxData {
        value: boolean | Boolean
        label: string
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
    import CheckboxItem from './checkbox-item.svelte'

    export let options: CheckboxData[]
    export let label: string
    export let description: string = ''
</script>

<fieldset class="flex flex-col">
    <legend class="">{label}</legend>

    {#if description}
        <p class="text-muted-foreground text-xs pt-1">
            {description}
        </p>
    {/if}

    <div class="h-[6px]"></div>

    <div class="flex gap-6 flex-wrap">
        {#each options as { onChange, ...props }}
            <CheckboxItem
                on:change={(ev) => onChange(ev.detail)}
                id={props.label}
                {...props}
            />
        {/each}
    </div>
</fieldset>

<style lang="postcss">
</style>
