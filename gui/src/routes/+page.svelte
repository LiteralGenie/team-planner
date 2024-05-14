<script lang="ts">
    import { setFilterFormContext } from '$lib/app/form-context/context'
    import { DEFAULT_FILTER_FORM } from '$lib/app/form-context/defaults'
    import ResultList from '$lib/app/result-list/result-list.svelte'
    import ResultCountInput from '$lib/app/search/result-count-input.svelte'
    import Search from '$lib/app/search/search.svelte'

    setFilterFormContext(DEFAULT_FILTER_FORM)
</script>

<main class="h-full w-full max-w-6xl mx-auto p-4 flex flex-col">
    <Search />

    <hr class="mt-8 pb-4" />

    <div class="w-full text-sm flex justify-end gap-4">
        <div class="result-count-select">
            <ResultCountInput />
        </div>
    </div>

    <ResultList />
</main>

<style lang="postcss">
    .result-count-select {
        @apply w-max text-muted-foreground;
    }
    .result-count-select :global(.sv-item--content) {
        @apply w-32;
    }
</style>
