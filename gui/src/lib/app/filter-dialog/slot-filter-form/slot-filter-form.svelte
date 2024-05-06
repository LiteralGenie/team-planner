<script lang="ts">
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

        <div>
            <h2 class="pb-1">Slot Preview</h2>

            <p class="text-muted-foreground text-xs pb-3">
                The settings above will allow the following champions
                for this slot.
            </p>

            <ChampionMatches filters={[]} />

            <div
                class="text-muted-foreground text-xs pt-4 flex gap-2 items-center"
            >
                <InfoIcon class="h-6 w-6" />

                <p>
                    To exclude a champion from this slot, try
                    switching the filter mode to champion.
                    <br /> To exclude a champion from all slots, see the
                    Global settings page.
                </p>
            </div>
        </div>
    </div>
</form>

<style lang="postcss">
</style>
