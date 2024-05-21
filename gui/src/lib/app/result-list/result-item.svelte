<script lang="ts" context="module">
</script>

<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import SpellTooltip from '$lib/components/spell-tooltip.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import { CHAMPIONS_BY_ID, TRAITS_BY_ID } from '$lib/constants'
    import {
        getTraitLevel,
        type TraitLevel
    } from '../form-context/utils'
    import ResultItemTrait from './result-item-trait.svelte'

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

    $: traits = Object.entries(traitCounts)
        .map(([id, count]) => {
            const trait = TRAITS_BY_ID[id]
            const level = getTraitLevel(count, trait)

            return {
                trait,
                level,
                count
            }
        })
        .filter(({ count }) => count > 1)
        .sort((left, right) => {
            const lvlDiff =
                scoreTraitLevel(left.level) -
                scoreTraitLevel(right.level)
            if (lvlDiff) {
                return lvlDiff
            }

            const countDiff = left.count - right.count
            return countDiff
        })
        .reverse()

    function scoreTraitLevel(level: TraitLevel | null): number {
        switch (level?.style_name) {
            case undefined:
                return 0
            case 'kBronze':
                return 1
            case 'kSilver':
                return 2
            case 'kGold':
                return 3
            case 'kChromatic':
                return 4
            default:
                console.error('Invalid trait level', level)
                throw Error()
        }
    }
</script>

<div class="card flex flex-col justify-center items-center gap-4">
    <!-- Champions -->
    <div class="flex gap-4 flex-wrap justify-center">
        {#each ids as id}
            <span class="h-12 w-12">
                <Tooltip.Root
                    group="spell"
                    openDelay={100}
                    closeOnPointerDown={true}
                    disableHoverableContent={true}
                >
                    <Tooltip.Trigger class="cursor-default">
                        <ChampionPortrait {id} />
                    </Tooltip.Trigger>
                    <Tooltip.Content class="spell-tooltip-container">
                        <SpellTooltip champion_id={id} />
                    </Tooltip.Content>
                </Tooltip.Root>
            </span>
        {/each}
    </div>

    <!-- Traits -->
    <div class="flex gap-4 flex-wrap justify-center">
        {#each traits as { trait, level, count }}
            <ResultItemTrait id={trait.trait_id} {level} {count} />
        {/each}
    </div>
</div>

<style lang="postcss">
    .card {
        @apply p-6 rounded-sm;

        background-color: hsl(var(--card) / 60%);
    }
</style>
