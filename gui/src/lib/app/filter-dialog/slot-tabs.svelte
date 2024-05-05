<script lang="ts">
    import { getFilterFormContext } from '../form-context/context'

    const { form } = getFilterFormContext()
</script>

<section class="border-r divider-color flex flex-col">
    <h1 class="font-bold pb-2">Filters</h1>

    <button>Global</button>

    {#each $form.slots as slot, idx}
        <button> Slot #{idx + 1} </button>
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
