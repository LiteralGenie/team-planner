<script lang="ts">
    import type { Team } from '$lib/assets/wasm/tft_core'
    import loadWasm, * as tft from '$lib/assets/wasm/tft_core'
    import { DerivedUniqueStore } from '$lib/utils/misc'
    import { onMount } from 'svelte'
    import { getFilterFormContext } from './form-context/context'
    import { doSearch } from './form-context/search'
    import type { FilterForm } from './form-context/types'

    const { formInitial } = getFilterFormContext()

    const uniqueForm = new DerivedUniqueStore(formInitial)

    let isMounted = false

    onMount(() => {
        isMounted = true
    })

    async function initAndSearch(
        form: FilterForm,
        isMounted: boolean
    ): Promise<Team[]> {
        if (!isMounted) {
            return []
        }

        // @fixme: this init logic shouldnt be this low in the render tree
        if (window.tft === undefined) {
            await loadWasm()
            window.tft = tft
        }

        return doSearch(form)
    }
</script>

{#await initAndSearch($uniqueForm, isMounted)}
    loading...
{:then results}
    {JSON.stringify(results, null, 4)}
{/await}

<br />
{JSON.stringify($uniqueForm, null, 4)}
