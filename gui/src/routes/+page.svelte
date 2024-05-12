<script lang="ts">
    import { setFilterFormContext } from '$lib/app/form-context/context'
    import { DEFAULT_FILTER_FORM } from '$lib/app/form-context/defaults'
    import ResultList from '$lib/app/result-list.svelte'
    import Search from '$lib/app/search/search.svelte'

    setFilterFormContext(DEFAULT_FILTER_FORM)
</script>

<main class="h-full w-full max-w-6xl mx-auto p-4 flex flex-col gap-4">
    <Search />

    <ResultList />
</main>

<style lang="postcss">
    .result {
        @apply min-h-24 flex flex-col justify-center;
    }

    .divider {
    }
</style>
