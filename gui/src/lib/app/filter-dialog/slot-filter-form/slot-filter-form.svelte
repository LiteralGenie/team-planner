<script lang="ts">
    import { apply_attribute_filter as applyAttributeFilter } from '$lib/app/form-context/utils'
    import AlertIcon from '$lib/icons/alert-icon.svelte'
    import InfoIcon from '$lib/icons/info-icon.svelte'
    import { getFilterFormContext } from '../../form-context/context'
    import ChampionMatches from './champion-matches.svelte'
    import CostInput from './cost-input.svelte'
    import DamageTypeInput from './damage-type-input.svelte'
    import RangeInput from './range-input.svelte'
    import SlotTypeInput from './slot-type-input.svelte'
    import TraitInput from './trait-input.svelte'

    export let slotIndex: number

    const { form, controls } = getFilterFormContext()
    $: slot = controls.slots.controls[slotIndex]
    $: attributeControls = slot.controls.byAttribute
    $: attributeValues = $form.slots[slotIndex].byAttribute

    $: slotMatches = applyAttributeFilter(attributeValues)
</script>

<form class="w-full pt-4">
    <div class="px-6 h-full w-full overflow-auto">
        <h1 class="text-xl text-foreground font-bold mb-4">
            Slot #{slotIndex + 1}
        </h1>

        <SlotTypeInput {slotIndex} />

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

        <hr class="my-6" />

        <div class="min-h-48 flex flex-col">
            <h2 class="pb-1">Slot Preview</h2>

            <p class="text-muted-foreground text-xs pb-3">
                Only teams containing at least one of these champions
                will be displayed.
            </p>

            {#if slotMatches.size > 0}
                <ChampionMatches attributeFilter={attributeValues} />

                <div
                    class="text-muted-foreground pt-3 flex gap-2 items-center flex-grow"
                >
                    <InfoIcon class="h-6 w-6" />

                    <p class="text-xs">
                        To exclude a champion from this slot, try
                        switching the filter mode to Champion.
                        <br /> To exclude a champion from all slots, see
                        the Global settings page.
                    </p>
                </div>
            {:else}
                <div
                    class="text-muted-foreground pt-3 flex gap-2 items-center"
                >
                    <AlertIcon class="h-6 w-6 text-destructive" />

                    <p>No champions match the specified filters.</p>
                </div>
            {/if}
        </div>
    </div>
</form>

<style lang="postcss">
</style>
