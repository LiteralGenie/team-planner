<script lang="ts">
    import Button from '$lib/components/ui/button/button.svelte'
    import { getFilterFormContext } from '../../form-context/context'
    import SlotTypeInput from '../slot-type-input.svelte'
    import ChampionPicker from './champion-picker.svelte'

    export let slotIndex: number

    const { resetSlotFilter } = getFilterFormContext()

    function handleReset() {
        resetSlotFilter(slotIndex)
    }
</script>

<form class="w-full pt-4">
    <div class="px-6 h-full overflow-auto">
        <div>
            <h1 class="text-xl text-foreground font-bold mb-2">
                Filters
            </h1>

            <SlotTypeInput {slotIndex} />
        </div>

        <hr class="my-6" />

        <ChampionPicker {slotIndex} />

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
