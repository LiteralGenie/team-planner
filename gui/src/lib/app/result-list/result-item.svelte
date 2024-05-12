<script lang="ts" context="module">
</script>

<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import TraitIcon from '$lib/components/trait-icon.svelte'
    import { CHAMPIONS_BY_ID, TRAITS_BY_ID } from '$lib/constants'
    import { sort } from 'radash'

    export let ids: string[]

    $: traitCounts = ids
        .map((id) => CHAMPIONS_BY_ID[id])
        .reduce(
            (acc, c) => {
                for (let trait of c.traits) {
                    if (!TRAITS_BY_ID[trait.id]) {
                        continue
                    }

                    acc[trait.id] = acc[trait.id] || 0
                    acc[trait.id] += 1
                }

                return acc
            },
            {} as Record<string, number>
        )

    $: traitCountEntriesSorted = sort(
        Object.entries(traitCounts),
        ([_, count]) => count,
        true
    )
</script>

<div class="card flex justify-between">
    <!-- Champions -->
    <div class="flex justify-start gap-2">
        {#each ids as id}
            <span class="h-12 w-12">
                <ChampionPortrait {id} />
            </span>
        {/each}
    </div>

    <!-- Traits -->
    <div class="flex gap-4">
        {#each traitCountEntriesSorted as [id, count]}
            <span
                class="inline-flex gap-1 justify-center items-center"
            >
                <div class="h-8 w-8">
                    <TraitIcon {id} />
                </div>

                <span>{count}</span>
            </span>
        {/each}
    </div>
</div>

<style lang="postcss">
    .card {
        @apply p-4 rounded-sm;

        background-color: hsl(var(--card) / 60%);
    }
</style>
