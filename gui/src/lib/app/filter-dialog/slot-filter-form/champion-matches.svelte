<script lang="ts">
    import type { AttributeFilter } from '$lib/app/form-context/types'
    import { apply_attribute_filter } from '$lib/app/form-context/utils'
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import { CHAMPIONS, CHAMPION_ICONS } from '$lib/constants'
    import { sort } from 'radash'

    export let attributeFilter: AttributeFilter

    $: activeIds = apply_attribute_filter(attributeFilter)

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
            <span
                class="text-xs text-muted-foreground whitespace-nowrap"
            >
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
        gap: 8px;
    }

    .cell {
        display: none;

        &.active {
            display: flex;
        }
    }
</style>
