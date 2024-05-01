<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import * as Card from '$lib/components/ui/card'
    import XIcon from '$lib/icons/x-icon.svelte'
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

    function handleCloseButtonClick() {
        dispatch('close')
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<dialog
    bind:this={dialogEl}
    on:click={handleBackdropClick}
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
                <fieldset class="flex gap-4">
                    <legend class="pb-2">Filter by:</legend>

                    <div>
                        <input
                            type="radio"
                            id="filter-attributes"
                            name="filter-type"
                            checked
                        />
                        <label for="filter-attributes">Role</label>
                    </div>

                    <div>
                        <input
                            type="radio"
                            id="filter-id"
                            name="filter-type"
                        />
                        <label for="filter-id">Champion</label>
                    </div>
                </fieldset>

                <hr class="my-8" />

                <!-- Costs -->
                <fieldset class="flex gap-4">
                    <legend class="pb-2">Tier</legend>

                    <div>
                        <input
                            type="checkbox"
                            id="tier-1"
                            name="tier-1"
                            checked
                        />
                        <!-- @TODO: COIN ICON -->
                        <label for="tier-1"> 1 cost </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="tier-2"
                            name="tier-2"
                            checked
                        />
                        <label for="tier-2"> 2 cost </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="tier-3"
                            name="tier-3"
                            checked
                        />
                        <label for="tier-3"> 3 cost </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="tier-4"
                            name="tier-4"
                            checked
                        />
                        <label for="tier-4"> 4 cost </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="tier-5"
                            name="tier-5"
                            checked
                        />
                        <label for="tier-5"> 5 cost </label>
                    </div>
                </fieldset>

                <!-- Ranges -->
                <fieldset class="mt-4 flex gap-4">
                    <legend class="pb-2">Range</legend>

                    <div>
                        <input
                            type="checkbox"
                            id="range-melee"
                            name="range-melee"
                            checked
                        />
                        <!-- @TODO: ICON -->
                        <label for="range-melee"> Melee </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="range-mid"
                            name="range-mid"
                            checked
                        />
                        <label for="range-mid"> Mid-range </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="range-long"
                            name="range-long"
                            checked
                        />
                        <label for="range-long"> Ranged </label>
                    </div>
                </fieldset>

                <!-- Damage Type -->
                <fieldset class="mt-4 flex gap-4">
                    <legend class="pb-2">Damage Type</legend>

                    <div>
                        <input
                            type="radio"
                            id="damage-ad"
                            name="damage-type"
                            checked
                        />
                        <label for="damage-ad">Physical</label>
                    </div>

                    <div>
                        <input
                            type="radio"
                            id="damage-ap"
                            name="damage-type"
                        />
                        <label for="damage-ap">Magical</label>
                    </div>
                </fieldset>

                <!-- Traits -->
                <fieldset class="mt-4 grid gap-4 grid-flow-col">
                    <legend class="pb-2">Traits</legend>

                    <!-- @TODO: trait icons -->
                    <div>
                        <input
                            type="checkbox"
                            id="trait-arcanist"
                            name="trait-arcanist"
                            checked
                        />
                        <label for="trait-arcanist"> Arcanist </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="trait-arcanist"
                            name="trait-arcanist"
                            checked
                        />
                        <label for="trait-arcanist"> Arcanist </label>
                    </div>

                    <div>
                        <input
                            type="checkbox"
                            id="trait-arcanist"
                            name="trait-arcanist"
                            checked
                        />
                        <label for="trait-arcanist"> Arcanist </label>
                    </div>
                </fieldset>

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
