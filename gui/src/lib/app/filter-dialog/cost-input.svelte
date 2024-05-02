<script lang="ts">
    import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte'
    import GoldIcon from '$lib/icons/gold-icon.svelte'
    import { createEventDispatcher } from 'svelte'

    export let value: boolean
    export let cost: number

    $: id = `cost-${cost}`

    const dispatch = createEventDispatcher()
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    on:click={() => dispatch('change', !value)}
    class="cursor-pointer flex justify-center items-center gap-2"
>
    <div class="checkbox-container flex items-center">
        <Checkbox checked={value} />
    </div>

    <label
        for={id}
        class="cursor-pointer flex items-center justify-center gap-1"
    >
        {cost}
        <GoldIcon class="w-3 h-3" />
    </label>

    <input type="hidden" {id} name={id} checked {value} />
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
