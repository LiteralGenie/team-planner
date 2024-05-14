<script lang="ts" context="module">
    const BATCH_SIZE = 100
    const FIRST_BATCH_SIZE = 50
    const NUM_BATCHES = 5

    interface ScoredTeam {
        team_id: number
        ids: string[]
        score: number
    }
</script>

<script lang="ts">
    import Loader from '$lib/components/loader.svelte'
    import { TRAITS_BY_ID } from '$lib/constants'
    import { DerivedUniqueStore } from '$lib/utils/misc'
    import { sort } from 'radash'
    import { getFilterFormContext } from '../form-context/context'
    import {
        getSearchResult as fetchSearchResults,
        setSearchOptions
    } from '../form-context/search'
    import type { FilterForm } from '../form-context/types'
    import { getTraitLevel, tallyTraits } from '../form-context/utils'
    import ResultItem from './result-item.svelte'

    const { formInitial } = getFilterFormContext()

    const uniqueForm = new DerivedUniqueStore(formInitial)

    let results: ScoredTeam[] = []
    let fetchId: number = 0
    let isFetching = false

    $: resultsSorted = sortResults(results).slice(0, 100)

    $: handleFormChange($uniqueForm)

    async function handleFormChange(form: FilterForm) {
        fetchId += 1
        results = []

        await setSearchOptions(form)
        await loadResultChunks(NUM_BATCHES, FIRST_BATCH_SIZE)
    }

    async function loadResultChunks(
        count: number,
        batchSizeOverride?: number
    ) {
        isFetching = true

        const idBeforeLoad = fetchId

        const teams = (
            await fetchSearchResults(batchSizeOverride ?? BATCH_SIZE)
        ).map((ids) => ({
            team_id: Math.random(),
            ids,
            score: scoreTeam(ids)
        }))
        results = [...results, ...teams]

        if (fetchId === idBeforeLoad && count > 1) {
            await loadResultChunks(count - 1)
        } else {
            isFetching = false
        }
    }

    function sortResults(results: ScoredTeam[]): ScoredTeam[] {
        const sorted = sort(results, (team) => team.score, true)
        return sorted
    }

    function scoreTeam(champions: string[]) {
        const traitCounts = tallyTraits(champions)

        let score = 0

        for (let x of Object.entries(traitCounts)) {
            const [traitId, count] = x
            const level = getTraitLevel(count, TRAITS_BY_ID[traitId])

            switch (level?.style_name) {
                case undefined:
                    score += 0
                    break
                case 'kBronze':
                    score += 1
                    break
                case 'kSilver':
                    score += 2
                    break
                case 'kGold':
                    score += 3
                    break
                case 'kChromatic':
                    score += 4
                    break
                default:
                    console.error('Invalid trait level', level)
                    throw Error()
            }
        }

        return score
    }
</script>

<div class="root pt-4">
    <!-- <h1 class="pb-4 text-xl font-bold">Matches</h1> -->

    {#if isFetching}
        <div class="loader flex justify-center pb-8">
            <Loader />
        </div>
    {/if}

    <div class="flex flex-col gap-4">
        {#each resultsSorted as { team_id, ids } (team_id)}
            <ResultItem {ids} />
        {/each}
    </div>
</div>

<br />

<style lang="postcss">
    .loader {
        fill: hsl(var(--foreground));
    }

    .loader :global(svg) {
        @apply h-8;
    }
</style>
