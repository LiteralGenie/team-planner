<script lang="ts" context="module">
</script>

<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import SpellTooltip from '$lib/components/spell-tooltip.svelte'
    import TraitIcon from '$lib/components/trait-icon.svelte'
    import TraitTooltip from '$lib/components/trait-tooltip.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import { CHAMPIONS_BY_ID, TRAITS_BY_ID } from '$lib/constants'
    import {
        getTraitLevel,
        type TraitLevel
    } from '../form-context/utils'

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
    <div class="flex justify-start gap-4">
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
    <div class="flex gap-4">
        {#each traits as { trait, level, count }}
            <span
                class:inactive={level === null}
                class="inline-flex gap-0 justify-center items-center"
            >
                <div class="icon">
                    <Tooltip.Root
                        openDelay={100}
                        closeOnPointerDown={true}
                        group="trait"
                        disableHoverableContent={true}
                    >
                        <Tooltip.Trigger class="h-[2.5em] w-[2.5em]">
                            <TraitIcon
                                id={trait.trait_id}
                                style={level?.style_name}
                                variant="md"
                            />
                        </Tooltip.Trigger>
                        <Tooltip.Content
                            class="boring-tooltip max-w-[400px]"
                        >
                            <TraitTooltip
                                trait_id={trait.trait_id}
                                traitLevelIdx={level?.levelIdx}
                            />
                        </Tooltip.Content>
                    </Tooltip.Root>
                </div>

                <span
                    class="label w-6 text-xs flex justify-center items-center"
                >
                    {count}
                </span>
            </span>
        {/each}
    </div>
</div>

<style lang="postcss">
    .card {
        @apply p-6 rounded-sm;

        background-color: hsl(var(--card) / 60%);
    }

    .label {
        border-width: 1.5px;
        border-left: none;
        border-color: #6b6d6b;
        height: 1.55em;
        width: 2em;
    }

    .icon :global(.outline-layer) {
        padding-right: 0;
    }

    .inactive {
        display: none;

        @media screen(md) {
            display: flex;
        }
    }
</style>
