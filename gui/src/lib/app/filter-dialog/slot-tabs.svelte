<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import { getFilterFormContext } from '../form-context/context'

    const { form } = getFilterFormContext()

    const dispatch = createEventDispatcher()
</script>

<section class="h-full border-r divider-color flex flex-col">
    <h1 class="font-bold pb-2">Filters</h1>

    <button on:click={() => dispatch('tabclick', 'global')}>
        Global
    </button>

    {#each $form.slots as slot, idx}
        <button on:click={() => dispatch('tabclick', idx)}>
            Slot #{idx + 1}
        </button>
    {/each}
</section>

<style lang="postcss">
    h1,
    button {
        @apply w-full px-6 text-left;
    }

    button {
        @apply text-sm py-2;
    }
</style>
