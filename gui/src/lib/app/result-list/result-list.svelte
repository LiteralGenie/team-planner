<script lang="ts">
    import { TRAITS_BY_ID } from '$lib/constants'
    import { DerivedUniqueStore } from '$lib/utils/misc'
    import { range, sort } from 'radash'
    import { onMount } from 'svelte'
    import { getFilterFormContext } from '../form-context/context'
    import {
        getSearchResult as fetchSearchResult,
        setSearchOptions
    } from '../form-context/search'
    import type { FilterForm } from '../form-context/types'
    import { getTraitLevel, tallyTraits } from '../form-context/utils'
    import ResultItem from './result-item.svelte'

    const { formInitial } = getFilterFormContext()

    const uniqueForm = new DerivedUniqueStore(formInitial)

    let isMounted = false

    onMount(() => {
        isMounted = true
    })

    async function initAndSearch(
        form: FilterForm,
        isMounted: boolean
    ): Promise<string[][]> {
        if (!isMounted) {
            return []
        }

        console.log('setting', form)
        setSearchOptions(form)

        const results: string[][] = []
        for (let _ of range(1000 - 1)) {
            const r = await fetchSearchResult()
            if (!r) {
                break
            }

            results.push(r)
        }

        return results
    }

    function sortResults(results: string[][]): string[][] {
        return sort(results, (cs) => scoreTeam(cs), true)

        function scoreTeam(champions: string[]) {
            const traitCounts = tallyTraits(champions)

            let score = 0

            for (let x of Object.entries(traitCounts)) {
                const [traitId, count] = x
                const level = getTraitLevel(
                    count,
                    TRAITS_BY_ID[traitId]
                )

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
    }
</script>

<!-- @todo: searches can block for several seconds so need to move it to a worker -->
{#await initAndSearch($uniqueForm, isMounted)}
    loading...
{:then results}
    <div class="pt-4">
        <!-- <h1 class="pb-4 text-xl font-bold">Matches</h1> -->

        <div class="flex flex-col gap-4">
            {#each sortResults(results).slice(100) as ids}
                <ResultItem {ids} />
            {/each}
        </div>
    </div>
{/await}

<br />
