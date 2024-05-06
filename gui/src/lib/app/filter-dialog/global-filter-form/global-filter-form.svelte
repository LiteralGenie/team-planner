<script lang="ts">
    import GlobalCostInput from './global-cost-input.svelte'
    import GlobalTraitInput from './global-trait-input.svelte'
</script>

<form class="h-full w-full pt-4 px-6">
    <h1 class="text-xl text-foreground font-bold mb-4">
        Global Filters
    </h1>

    <div class="flex flex-col gap-6">
        <GlobalCostInput />
        <GlobalTraitInput />
    </div>
</form>

<style lang="postcss">
</style>
