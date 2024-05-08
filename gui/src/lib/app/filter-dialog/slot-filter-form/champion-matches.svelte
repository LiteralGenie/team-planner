<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { AttributeFilter } from '$lib/app/form-context/types'
    import { applyAttributeFilter } from '$lib/app/form-context/utils'
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import { CHAMPIONS, CHAMPION_ICONS } from '$lib/constants'
    import { sort } from 'radash'

    export let attributeFilter: AttributeFilter

    const { form } = getFilterFormContext()

    $: activeIds = applyAttributeFilter($form.global, attributeFilter)

    $: championsSorted = sort(
        CHAMPIONS,
        (c) => +activeIds.has(c.character_id),
        true
    )
</script>

<div class="icon-grid">
    {#each championsSorted as c}
        <div
            class="cell flex flex-col justify-center items-center text-center gap-[1px]"
            class:active={activeIds.has(c.character_id)}
        >
            <div class="h-12 w-12 relative select-none">
                <ChampionPortrait
                    src={CHAMPION_ICONS[c.character_id]}
                    cost={c.tier}
                />
            </div>

            <!-- Label -->
            <span class="text-xs whitespace-nowrap">
                {c.display_name}
            </span>
        </div>
    {/each}
</div>

<style lang="postcss">
    /* @todo how to make all rows the same height when one row has text wrap */
    .icon-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
        align-items: start;
        gap: 6px;
    }

    .cell {
        display: none;

        &.active {
            display: flex;
        }
    }
</style>
