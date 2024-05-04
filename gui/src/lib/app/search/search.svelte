<script lang="ts">
    import * as Card from '$lib/components/ui/card'
    import { someFalse } from '$lib/utils/misc'
    import Button from '../../components/ui/button/button.svelte'
    import FilterDialog from '../filter-dialog/filter-dialog.svelte'
    import { getFilterFormContext } from '../form-context/context'
    import type { SlotFilter } from '../form-context/types'
    import FilterButton from './filter-button.svelte'
    import FilterPreview from './filter-preview.svelte'

    let showDialog = false
    let activeDialogSlot = 0

    const { form, controls } = getFilterFormContext()
    $: console.log('form value', $form)

    function handleDialogOpen(idx: number) {
        activeDialogSlot = idx
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

<div>
    <FilterDialog
        open={showDialog}
        slotIndex={activeDialogSlot}
        on:close={handleDialogClose}
    />

    <Card.Root
        class="w-full rounded-sm flex flex-col justify-center gap-4"
    >
        <form class="w-full filter-grid">
            {#each $form.slots as slot, idx}
                <div
                    class="p-4 pl-8 border cell-border rounded-md flex gap-4 items-center"
                >
                    <FilterButton
                        on:click={() => handleDialogOpen(idx)}
                        variant={slotState(slot)}
                    />
                    <FilterPreview {slot} />
                </div>
            {/each}
        </form>

        <section class="w-full p-8 flex justify-end">
            <Button class="w-24">Search -></Button>
        </section>
    </Card.Root>
</div>

<style lang="postcss">
    .filter-grid {
        display: grid;
        gap: 8px;
        padding: 8px;
        grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    }

    .cell-border {
        border-color: hsl(var(--foreground) / 30%);
    }
</style>
