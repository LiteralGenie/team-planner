<script lang="ts" context="module">
    export interface OptionData {
        id: string
        label: string
        value: boolean
    }
</script>

<script lang="ts">
    import * as RadioGroup from '$lib/components/ui/radio-group'
    import { createEventDispatcher } from 'svelte'
    import BoolOptionItem from './bool-option-item.svelte'

    export let options: OptionData[]
    export let value: boolean

    let dispatch = createEventDispatcher()

    function handleSelect(selection: boolean) {
        dispatch('select', selection)
    }
</script>

<RadioGroup.Root
    value={String(value)}
    class="flex items-center justify-center gap-4 flex-wrap"
>
    {#each options as opt}
        <BoolOptionItem
            on:select={() => handleSelect(opt.value)}
            {...opt}
        />
    {/each}
</RadioGroup.Root>
