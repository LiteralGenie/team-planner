<script lang="ts">
    import { CHAMPIONS } from '$lib/constants'
    import FilterDialog from '../filter-dialog/filter-dialog.svelte'
    import type { SlotIndex } from '../filter-dialog/slot-tabs.svelte'
    import { getFilterFormContext } from '../form-context/context'
    import type { SlotFilter } from '../form-context/types'
    import { applyAttributeFilter } from '../form-context/utils'
    import FilterButton from './filter-button/filter-button.svelte'
    import FilterPreview from './filter-button/filter-preview.svelte'
    import GlobalFilterButton from './global-filter-button/global-filter-button.svelte'
    import GlobalFilterPreview from './global-filter-button/global-filter-preview.svelte'

    let showDialog = false
    let activeSlotIndex: SlotIndex = 0

    const { form, controls } = getFilterFormContext()

    function handleDialogOpen(idx: SlotIndex) {
        activeSlotIndex = idx
        showDialog = true
    }

    function handleDialogClose() {
        showDialog = false
    }

    function slotState(slot: SlotFilter): string {
        if (slot.useAttributes) {
            const matches = applyAttributeFilter(slot.byAttribute)

            if (matches.size === 0) {
                return 'active'
            } else if (matches.size === 1) {
                // showing champion icon is possible but looks janky due to how large the container is + how zoomed in the image is
                // return matches.values().next().value
                return 'active'
            } else if (matches.size === CHAMPIONS.length) {
                return 'inactive'
            } else {
                return 'active'
            }
        } else {
            if (slot.byId.length > 1) {
                return 'active'
            } else if (slot.byId.length === 1) {
                // return slot.byId[0].id
                return 'active'
            } else {
                return 'inactive'
            }
        }
    }
</script>

<div class="root">
    <FilterDialog
        open={showDialog}
        slotIndex={activeSlotIndex}
        on:close={handleDialogClose}
        on:tabclick={(ev) => handleDialogOpen(ev.detail)}
    />

    <div class="card w-full rounded-sm flex flex-col justify-center">
        <div class="p-2 flex flex-col gap-2">
            <div class="flex flex-col gap-2">
                <div
                    class="filter-grid text-sm text-muted-foreground"
                >
                    {#each $form.slots as slot, idx}
                        <div class="cell flex gap-4 items-center">
                            <FilterButton
                                on:click={() => handleDialogOpen(idx)}
                                variant={slotState(slot)}
                            />
                            <FilterPreview {slot} />
                        </div>
                    {/each}
                </div>
            </div>

            <div
                class="cell !p-1 text-muted-foreground text-sm flex gap-2 items-center"
            >
                <GlobalFilterButton
                    on:click={() => handleDialogOpen('global')}
                />

                <GlobalFilterPreview />
            </div>
        </div>

        <!-- <section class="w-full p-8 flex justify-end">
            <div><Button class="w-24">Search -></Button></div>
        </section> -->
    </div>
</div>

<style lang="postcss">
    .card {
        background-color: hsl(var(--card) / 60%);
    }

    .filter-grid {
        display: grid;
        gap: 8px;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    }

    .cell {
        @apply p-4 pl-8 border rounded-md;
        background-color: hsl(var(--foreground) / 8%);
    }

    .root :global(.settings-button) {
        background-color: hsl(var(--background) / 90%);
    }
</style>
