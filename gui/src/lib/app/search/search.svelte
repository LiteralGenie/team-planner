<script lang="ts">
    import CogIcon from '$lib/icons/cog-icon.svelte'
    import { someFalse } from '$lib/utils/misc'
    import Button from '../../components/ui/button/button.svelte'
    import FilterDialog from '../filter-dialog/filter-dialog.svelte'
    import { getFilterFormContext } from '../form-context/context'
    import type { SlotFilter } from '../form-context/types'
    import FilterButton from './filter-preview/filter-button.svelte'
    import FilterPreview from './filter-preview/filter-preview.svelte'

    let showDialog = false
    let activeSlotIndex: number | 'global' = 0

    const { form, controls } = getFilterFormContext()

    function handleDialogOpen(idx: number | 'global') {
        activeSlotIndex = idx
        showDialog = true
    }

    function handleDialogClose() {
        showDialog = false
    }

    function slotState(slot: SlotFilter): string {
        if (slot.useAttributes) {
            const attrs = slot.byAttribute

            if (someFalse(attrs.cost)) {
                return 'active'
            } else if (someFalse(attrs.damageType)) {
                return 'active'
            } else if (someFalse(attrs.range)) {
                return 'active'
            } else if (attrs.traits.some((t) => t.state !== 0)) {
                return 'active'
            } else {
                return 'inactive'
            }
        } else {
            if (slot.byId.length > 1) {
                return 'active'
            } else if (slot.byId.length === 1) {
                return slot.byId[0]
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
                <Button
                    on:click={() => handleDialogOpen('global')}
                    class="settings-button text-foreground px-3 flex gap-2"
                    variant="outline"
                >
                    <span class="flex gap-1 items-center">
                        <CogIcon class="h-4 w-4" />
                    </span>
                </Button>

                <div>Global Filters: None</div>
            </div>
        </div>

        <section class="w-full p-8 flex justify-end">
            <div><Button class="w-24">Search -></Button></div>
        </section>
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
