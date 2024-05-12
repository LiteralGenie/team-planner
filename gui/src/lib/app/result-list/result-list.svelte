<script lang="ts">
    import loadWasm, * as tft from '$lib/assets/wasm/tft_core'
    import { DerivedUniqueStore } from '$lib/utils/misc'
    import { shuffle } from 'radash'
    import { onMount } from 'svelte'
    import { getFilterFormContext } from '../form-context/context'
    import { doSearch } from '../form-context/search'
    import type { FilterForm } from '../form-context/types'
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

        // @todo: this init-if-not-ssr logic should be higher in the render tree
        if (window.tft === undefined) {
            await loadWasm()
            window.tft = tft
        }

        const results = doSearch(form)

        // @todo: rust should be randomizing this
        // @todo: control result count from gui
        return shuffle(results).slice(0, 40)
    }
</script>

<!-- @todo: searches can block for several seconds so need to move it to a worker -->
{#await initAndSearch($uniqueForm, isMounted)}
    loading...
{:then results}
    <div class="pt-4">
        <!-- <h1 class="pb-4 text-xl font-bold">Matches</h1> -->

        <div class="flex flex-col gap-4">
            {#each results as ids}
                <ResultItem {ids} />
            {/each}
        </div>
    </div>
{/await}

<br />
