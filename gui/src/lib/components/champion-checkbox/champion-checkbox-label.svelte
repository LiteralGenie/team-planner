<script lang="ts" context="module">
    export type ChampionCheckboxValue = 'included' | 'excluded' | null
</script>

<script lang="ts">
    import { CHAMPIONS_BY_ID } from '$lib/constants'

    export let id: string
    export let value: ChampionCheckboxValue
    export let disabled = false
    export let disabledValue: ChampionCheckboxValue = 'excluded'

    $: label = CHAMPIONS_BY_ID[id].display_name

    $: actualValue = disabled ? disabledValue : value
</script>

<span
    class:disabled
    class:active={actualValue !== null}
    class="text-xs text-muted-foreground whitespace-nowrap"
>
    {label}
</span>

<style lang="postcss">
    span.active:not(.disabled) {
        @apply text-foreground;
    }

    /* Prevent text wrap from creating uneven row heights */
    span {
        overflow: hidden;
        white-space: nowrap;
    }
</style>
