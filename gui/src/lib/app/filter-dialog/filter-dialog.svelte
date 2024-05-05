<script lang="ts">
    import XIcon from '$lib/icons/x-icon.svelte'
    import { createEventDispatcher } from 'svelte'
    import GlobalFilterForm from './global-filter-form/global-filter-form.svelte'
    import SlotFilterForm from './slot-filter-form/slot-filter-form.svelte'
    import SlotTabs, { type SlotIndex } from './slot-tabs.svelte'

    export let open = false
    export let slotIndex: SlotIndex

    let dispatch = createEventDispatcher()

    let dialogEl: HTMLDialogElement
    $: open ? dialogEl?.showModal() : dialogEl?.close()

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
        class="close-icon absolute top-3 right-3 p-2"
        on:click={handleCloseButtonClick}
    >
        <XIcon class="h-5 w-5" />
    </button>

    <div
        class="pt-12 pb-12 card rounded-2xl h-full w-full text-muted-foreground flex flex-col"
    >
        <div class="h-full flex">
            <div class="hidden md:block">
                <SlotTabs {slotIndex} on:tabclick />
            </div>

            <section class="w-full min-w-0 pr-6 flex">
                {#if typeof slotIndex === 'number'}
                    <SlotFilterForm {slotIndex} />
                {:else}
                    <GlobalFilterForm />
                {/if}
            </section>
        </div>
    </div>
</dialog>

<style lang="postcss">
    .close-icon {
        color: hsl(var(--foreground));
    }

    .card {
        background-color: hsl(var(--card));
    }
</style>
