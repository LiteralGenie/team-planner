<script lang="ts" context="module">
    import type { SvelteComponent } from 'svelte'

    export interface CheckboxData {
        value: boolean | Boolean
        label: string
        suffix?: typeof SvelteComponent<any>
        onChange: (update: boolean) => void
    }
</script>

<script lang="ts">
    import CheckboxItem from './checkbox-item.svelte'

    export let options: CheckboxData[]
    export let label: string
</script>

<fieldset class="flex">
    <legend class="pb-[6px]">{label}</legend>

    <div class="flex gap-6">
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
