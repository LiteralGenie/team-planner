<script lang="ts">
    import { someFalse } from '$lib/utils/misc'
    import Button from '../../components/ui/button/button.svelte'
    import * as Card from '../../components/ui/card'
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

    <Card.Root class="w-full p-8 flex flex-col justify-center gap-4">
        <form
            class="w-full py-4 flex justify-center items-center gap-x-12 gap-y-4 flex-wrap"
        >
            {#each $form.slots as slot, idx}
                <div class="flex gap-2 justify-center items-center">
                    <FilterButton
                        on:click={() => handleDialogOpen(idx)}
                        variant={slotState(slot)}
                    />
                    <FilterPreview {slot} />
                </div>
            {/each}
        </form>

        <section class="w-full flex justify-end">
            <Button class="w-24">Search -></Button>
        </section>
    </Card.Root>
</div>
