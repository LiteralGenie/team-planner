<script lang="ts">
    import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte'
    import { SvelteComponent, createEventDispatcher } from 'svelte'

    export let id: string
    export let label: string
    export let value: boolean | Boolean
    export let suffix: typeof SvelteComponent | null = null
    export let suffixOpts: any = null
    export let suffixClass: string = 'h-3 w-3'
    export let prefix: typeof SvelteComponent | null = null
    export let prefixOpts: any = null
    export let prefixClass: string = 'h-3 w-3'

    $: v = value as boolean // see BoolParser

    const dispatch = createEventDispatcher()
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    on:click={() => dispatch('change', !value)}
    class="cursor-pointer flex items-center gap-2"
>
    <div class="checkbox-container flex items-center">
        <Checkbox checked={v} />
    </div>

    <label
        for={id}
        class="cursor-pointer flex items-center justify-center gap-1 text-[13px]"
    >
        {#if prefix}
            <svelte:component
                this={prefix}
                class={prefixClass}
                {...prefixOpts}
            />
        {/if}
        {label}
        {#if suffix}
            <svelte:component
                this={suffix}
                class={suffixClass}
                {...suffixOpts}
            />
        {/if}
    </label>

    <input type="hidden" {id} name={id} {value} />
</div>

<style>
    /* Manually center checkbox. Its actual bounds doesn't match visual bounds. */
    .checkbox-container {
        padding-left: 2px;
        padding-top: 1px;
    }

    label {
        color: hsl(var(--foreground));
    }
</style>
