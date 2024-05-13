<script lang="ts" context="module">
</script>

<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import TraitIcon from '$lib/components/trait-icon.svelte'
    import { CHAMPIONS_BY_ID, TRAITS_BY_ID } from '$lib/constants'
    import { sort } from 'radash'
    import { getTraitLevel } from '../form-context/utils'

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

    $: traits = sort(
        Object.entries(traitCounts),
        ([_, count]) => count,
        true
    ).map(([id, count]) => {
        const trait = TRAITS_BY_ID[id]
        const level = getTraitLevel(count, trait)

        return {
            trait,
            level,
            count
        }
    })
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
        {#each traits as { trait, level, count }}
            <span
                class="inline-flex gap-0 justify-center items-center"
            >
                <div class="h-9 w-9">
                    <TraitIcon
                        id={trait.trait_id}
                        style={level?.style_name}
                        variant="md"
                    />
                </div>

                <span
                    class="w-6 h-5 text-xs border-2 border-l-0 text-center"
                >
                    {count}
                </span>
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
