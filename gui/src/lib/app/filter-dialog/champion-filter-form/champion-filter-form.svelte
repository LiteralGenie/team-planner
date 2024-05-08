<script lang="ts">
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type { IdFilter } from '$lib/app/form-context/types'
    import Button from '$lib/components/ui/button/button.svelte'
    import {
        CHAMPIONS_BY_ID,
        type CDragonChampion
    } from '$lib/constants'
    import { zip } from 'radash'
    import { derived } from 'svelte/store'
    import { getFilterFormContext } from '../../form-context/context'
    import SlotTypeInput from '../slot-type-input.svelte'
    import ChampionPicker from './champion-picker.svelte'

    export let slotIndex: number

    const { form, controls, resetSlotFilter } = getFilterFormContext()
    $: slotValues = $form.slots[slotIndex].byId
    $: slotControls = derived(
        controls.slots.controlsStore,
        (controls) => controls[slotIndex].controls.byId.controls
    )
    $: zipped = zip(slotValues, $slotControls).map(([val, ctrl]) => [
        val,
        ctrl,
        CHAMPIONS_BY_ID[val.id]
    ]) as Array<
        [IdFilter, FormControlRecord<IdFilter>, CDragonChampion]
    >

    function handleClick(
        current: IdFilter,
        ctrl: FormControlRecord<IdFilter>
    ) {
        ctrl.onChange({
            ...current,
            included: !current.included
        })
    }

    function handleReset() {
        resetSlotFilter(slotIndex)
    }

    function isDisabledGlobally(id_champion: string) {
        const globalChampion = $form.global.champions.find(
            ({ id }) => id === id_champion
        )!

        return !globalChampion.included
    }
</script>

<form class="w-full pt-4">
    <div class="px-6 h-full overflow-auto">
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-xl text-foreground font-bold mb-2">
                    Filters
                </h1>

                <SlotTypeInput {slotIndex} />
            </div>

            <Button on:click={handleReset} class="px-6">Reset</Button>
        </div>

        <hr class="my-6" />

        <ChampionPicker {slotIndex} />
    </div>
</form>

<style lang="postcss">
    .champion-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
        align-items: start;
        gap: 12px 12px;
    }
</style>
