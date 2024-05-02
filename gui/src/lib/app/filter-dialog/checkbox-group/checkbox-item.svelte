<script lang="ts">
    import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte'
    import { SvelteComponent, createEventDispatcher } from 'svelte'

    export let id: string
    export let label: string
    export let value: boolean | Boolean
    export let suffix: typeof SvelteComponent | null = null

    $: v = value as boolean // see BoolParser

    const dispatch = createEventDispatcher()
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    on:click={() => dispatch('change', !value)}
    class="cursor-pointer flex justify-center items-center gap-2"
>
    <div class="checkbox-container flex items-center">
        <Checkbox checked={v} />
    </div>

    <label
        for={id}
        class="cursor-pointer flex items-center justify-center gap-1"
    >
        {label}
        {#if suffix}
            <svelte:component this={suffix} class="w-3 h-3" />
        {/if}
    </label>

    <input type="hidden" {id} name={id} {value} />
</div>

<style>
    /* Manually center checkbox. Its actual bounds doesn't match visual bounds. */
    .checkbox-container {
        padding-top: 1px;
    }

    label {
        color: hsl(var(--foreground));
    }
</style>
