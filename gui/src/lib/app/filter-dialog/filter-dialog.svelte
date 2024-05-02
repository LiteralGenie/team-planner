<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import * as Card from '$lib/components/ui/card'
    import XIcon from '$lib/icons/x-icon.svelte'
    import { createEventDispatcher } from 'svelte'
    import { getFilterFormContext } from '../form-context/context'
    import CostInputGroup from './cost-input-group.svelte'
    import DamageTypeInput from './damage-type-input.svelte'
    import RangeInput from './range-input.svelte'
    import SlotTypeInput from './slot-type-input.svelte'
    import TraitInput from './trait-input.svelte'

    export let open = false
    export let slotIndex: number

    let dispatch = createEventDispatcher()

    let dialogEl: HTMLDialogElement
    $: open ? dialogEl?.showModal() : dialogEl?.close()

    const { form, controls } = getFilterFormContext()
    $: slot = controls.slots.controls[slotIndex]
    $: attributeSlot = slot.controls.byAttribute

    function handleBackdropClick(ev: MouseEvent) {
        // This will only trigger on backdrop clicks, not dialog content clicks
        // Because in the latter case, the event target will be one of the inner elements
        if (ev.target === dialogEl) {
            dispatch('close')
        }
    }

    function handleCloseButtonClick() {
        dispatch('close')
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<dialog
    bind:this={dialogEl}
    on:click={handleBackdropClick}
    on:close
    class="h-full w-full max-h-[80vh] max-w-[80vw] bg-transparent"
>
    <button
        class="close-icon absolute top-4 right-4 p-2"
        on:click={handleCloseButtonClick}
    >
        <XIcon class="h-6 w-6" />
    </button>
    <Card.Root class="h-full w-full p-8 pb-6 flex flex-col gap-4">
        <Card.Title class="text-xl">Champion Filters</Card.Title>
        <Card.Description class="h-full">
            <form>
                <SlotTypeInput {slotIndex} />

                <hr class="my-8" />

                <CostInputGroup slot={attributeSlot} />

                <RangeInput />

                <DamageTypeInput />

                <TraitInput />

                <hr class="my-8" />

                Matching champions:
            </form>
        </Card.Description>
        <Card.Footer class="p-0 flex justify-end">
            <Button
                on:click={handleCloseButtonClick}
                class="min-w-24 min-h-10">Close</Button
            >
        </Card.Footer>
    </Card.Root>
</dialog>

<style lang="postcss">
    .close-icon {
        color: hsl(var(--foreground));
    }

    hr {
        border-color: hsl(var(--foreground) / 10%);
    }
</style>
