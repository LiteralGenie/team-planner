<script lang="ts">
    import { TRAITS_BY_ID } from '$lib/constants'
    import type { TraitCheckboxValue } from './trait-checkbox.svelte'

    export let id: string
    export let value: TraitCheckboxValue
    export let disabled = false
    export let disabledValue: TraitCheckboxValue = 'excluded'

    $: label = TRAITS_BY_ID[id].display_name
    $: actualValue = disabled ? disabledValue : value
</script>

<span
    class:active={actualValue !== null}
    class:disabled
    class="text-xs text-muted-foreground whitespace-nowrap"
>
    {label}
</span>

<style lang="postcss">
    span.active:not(.disabled) {
        @apply text-foreground;
    }
</style>
