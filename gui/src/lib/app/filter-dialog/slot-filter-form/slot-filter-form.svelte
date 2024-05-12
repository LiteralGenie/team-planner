<script lang="ts">
    import { applyAttributeFilterWithGlobal } from '$lib/app/form-context/utils'
    import Button from '$lib/components/ui/button/button.svelte'
    import AlertIcon from '$lib/icons/alert-icon.svelte'
    import InfoIcon from '$lib/icons/info-icon.svelte'
    import { onMount } from 'svelte'
    import { derived } from 'svelte/store'
    import { getFilterFormContext } from '../../form-context/context'
    import SlotTypeInput from '../slot-type-input.svelte'
    import ChampionMatches from './champion-matches.svelte'
    import CostInput from './cost-input.svelte'
    import DamageTypeInput from './damage-type-input.svelte'
    import RangeInput from './range-input.svelte'
    import TraitInput from './trait-input.svelte'

    export let slotIndex: number

    $: {
        slotIndex && resetScroll()
    }

    let mainScrollEl: HTMLElement
    let mobileFilterScrollEl: HTMLElement
    let mobilePreviewScrollEl: HTMLElement

    const { form, controls, resetSlotFilter } = getFilterFormContext()
    $: slot = derived(
        controls.slots.controlsStore,
        (controls) => controls[slotIndex]
    )
    $: attributeControls = $slot.controls.byAttribute
    $: attributeValues = $form.slots[slotIndex].byAttribute

    $: slotMatches = applyAttributeFilterWithGlobal(
        $form.global,
        attributeValues
    )

    function resetScroll() {
        mainScrollEl?.scrollTo({ top: 0 })
        mobileFilterScrollEl?.scrollTo({ top: 0 })
        mobilePreviewScrollEl?.scrollTo({ top: 0 })
    }

    function handleReset() {
        resetSlotFilter(slotIndex)
    }

    onMount(() => {
        resetScroll()
    })
</script>

<form class="w-full pt-4">
    <div
        bind:this={mainScrollEl}
        class="conditional-grid px-6 h-full w-full overflow-auto"
    >
        <!-- Filters -->
        <div bind:this={mobileFilterScrollEl} class="filters">
            <div>
                <h1 class="text-xl text-foreground font-bold mb-2">
                    Filters
                </h1>

                <SlotTypeInput {slotIndex} />
            </div>

            <hr class="my-6" />

            <div class="flex flex-col gap-6">
                <CostInput
                    slotControls={attributeControls}
                    slotValues={attributeValues}
                />

                <RangeInput
                    slotControls={attributeControls}
                    slotValues={attributeValues}
                />

                <DamageTypeInput
                    slotControls={attributeControls}
                    slotValues={attributeValues}
                />

                <TraitInput
                    slotControls={attributeControls}
                    slotValues={attributeValues}
                />
            </div>
        </div>

        <hr class="my-6 conditional-divider" />

        <!-- Preview -->
        <div
            bind:this={mobilePreviewScrollEl}
            class="preview min-h-48 flex flex-col"
        >
            <h1 class="pb-1 text-xl font-bold">Slot Preview</h1>

            <p class="text-muted-foreground text-xs pb-3">
                Only teams containing at least one of these champions
                will be displayed.
            </p>

            <div class="flex-grow">
                {#if slotMatches.size > 0}
                    <ChampionMatches
                        attributeFilter={attributeValues}
                    />
                {:else}
                    <div
                        class="text-muted-foreground pt-3 flex gap-2 items-center"
                    >
                        <AlertIcon class="h-6 w-6 text-destructive" />

                        <p>
                            No champions match the specified filters!
                        </p>
                    </div>
                {/if}
            </div>

            <div
                class="hint text-muted-foreground pt-6 flex gap-2 items-center"
            >
                <InfoIcon class="h-6 w-6" />

                <p class="text-xs">
                    To exclude a champion from this slot, try
                    switching the filter mode to Champion.
                    <br /> To exclude a champion from all slots, see the
                    Global filter tab.
                </p>
            </div>
        </div>

        <div class="pt-8 pr-2 col-span-2 flex justify-end">
            <Button
                variant="secondary"
                on:click={() => handleReset()}
                class="min-w-full lg:min-w-max px-6"
            >
                Clear
            </Button>
        </div>
    </div>
</form>

<style lang="postcss">
    @media screen(lg) {
        .conditional-grid {
            display: grid;
            grid-template-columns: minmax(350px, 50%) 1fr;
            grid-template-rows: 1fr max-content;
        }

        .filters {
            display: flex;
            flex-flow: column;
            overflow-y: auto;
            overflow-x: hidden;
            padding-right: 1em;
        }

        .conditional-divider {
            display: none;
        }

        .preview {
            overflow: auto;
            margin: 0 1em;
            padding: 0 1em;
        }
    }
</style>
