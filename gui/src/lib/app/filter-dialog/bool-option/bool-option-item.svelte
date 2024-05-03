<script lang="ts">
    import Label from '$lib/components/ui/label/label.svelte'
    import * as RadioGroup from '$lib/components/ui/radio-group'
    import { createEventDispatcher } from 'svelte'

    export let id: string
    export let label: string
    export let value: boolean

    let dispatch = createEventDispatcher()

    function handleSelect(ev: Event) {
        dispatch('select')
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    on:click={handleSelect}
    class="root h-full cursor-pointer flex items-center space-x-2"
>
    <RadioGroup.Item {id} value={String(value)} />
    <Label for={id} class="cursor-pointer">{label}</Label>
</div>

<style lang="postcss">
    .root :global(label) {
        color: hsl(var(--foreground));
    }
</style>
