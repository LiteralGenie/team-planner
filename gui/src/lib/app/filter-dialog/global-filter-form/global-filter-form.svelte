<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import Button from '$lib/components/ui/button/button.svelte'
    import GlobalChampionInput from './global-champion-input.svelte'
    import GlobalCostInput from './global-cost-input.svelte'
    import GlobalTraitInput from './global-trait-input.svelte'

    const { resetGlobalFilter } = getFilterFormContext()
</script>

<form class="h-full w-full pt-4 px-6 overflow-auto">
    <h1 class="text-xl text-foreground font-bold mb-4">
        Global Filters
    </h1>

    <div class="flex flex-col gap-6">
        <GlobalCostInput />
        <GlobalTraitInput />
        <GlobalChampionInput />
    </div>

    <div class="pt-8 flex justify-end">
        <Button
            variant="secondary"
            on:click={resetGlobalFilter}
            class="min-w-full lg:min-w-max px-6"
        >
            Clear
        </Button>
    </div>
</form>

<style lang="postcss">
</style>
