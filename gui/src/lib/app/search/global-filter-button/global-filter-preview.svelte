<script lang="ts" context="module">
</script>

<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import GoldIcon from '$lib/icons/gold-icon.svelte'

    const { form } = getFilterFormContext()

    $: bannedCostTiers = Object.entries($form.global.cost)
        .filter(([cost, val]) => !val)
        .map(([cost, _]) => cost)

    $: emptyFilters = bannedCostTiers.length === 0
</script>

<div>
    <h1 class="inline pr-1">Global Filters:</h1>

    {#if emptyFilters}
        <span>None</span>
    {:else if bannedCostTiers.length > 0}
        <div class="inline-flex gap-1 items-center">
            <span class="relative">
                <div class="absolute h-full w-full slash"></div>
                <GoldIcon class="h-[0.75em] w-[0.75em]" />
            </span>
            <span class="comma-group">
                {#each bannedCostTiers as cost}
                    <span>{cost}</span>
                {/each}
            </span>
        </div>
    {/if}
</div>

<style lang="postcss">
    .slash {
        background: linear-gradient(
            to top left,
            rgba(0, 0, 0, 0) 0%,
            rgba(0, 0, 0, 0) calc(50% - 1.1px),
            red 50%,
            rgba(0, 0, 0, 0) calc(50% + 1.1px),
            rgba(0, 0, 0, 0) 100%
        );
    }

    .comma-group {
        & > *:not(:last-child)::after {
            content: ', ';
        }
    }
</style>
