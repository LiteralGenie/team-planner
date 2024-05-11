<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { AttributeFilter } from '$lib/app/form-context/types'
    import { applyAttributeFilterWithGlobal } from '$lib/app/form-context/utils'
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import SpellTooltip from '$lib/components/spell-tooltip.svelte'
    import Button from '$lib/components/ui/button/button.svelte'
    import * as Popover from '$lib/components/ui/popover/index.js'
    import { CHAMPIONS, CHAMPION_ICONS } from '$lib/constants'
    import { sort } from 'radash'

    export let attributeFilter: AttributeFilter

    const { form } = getFilterFormContext()

    $: activeIds = applyAttributeFilterWithGlobal(
        $form.global,
        attributeFilter
    )

    $: championsSorted = sort(
        CHAMPIONS,
        (c) => +activeIds.has(c.character_id),
        true
    )
</script>

<div class="icon-grid gap-2">
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
            <span class="hidden md:block text-xs whitespace-nowrap">
                {c.display_name}
            </span>

            <!-- Mobile label with popover description -->
            <span class="md:hidden">
                <Popover.Root portal={'dialog'}>
                    <Popover.Trigger asChild let:builder>
                        <Button
                            builders={[builder]}
                            variant="link"
                            class="h-6 text-inherit"
                        >
                            {c.display_name}
                        </Button>
                    </Popover.Trigger>
                    <Popover.Content class="w-full max-w-[500px]">
                        <SpellTooltip champion_id={c.character_id} />
                    </Popover.Content>
                </Popover.Root>
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
    }

    .cell {
        display: none;

        &.active {
            display: flex;
        }
    }
</style>
