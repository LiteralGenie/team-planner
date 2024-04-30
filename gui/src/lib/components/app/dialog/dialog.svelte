<script lang="ts">
    import { createEventDispatcher } from 'svelte'

    export let open = false

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
</script>

<dialog bind:this={dialogEl} on:click={handleBackdropClick}>
    <div>test</div>
</dialog>

<style lang="postcss">
</style>
