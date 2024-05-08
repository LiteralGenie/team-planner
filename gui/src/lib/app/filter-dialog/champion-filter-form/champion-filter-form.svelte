<script lang="ts">
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type { IdFilter } from '$lib/app/form-context/types'
    import ChampionCheckbox from '$lib/components/champion-checkbox.svelte'
    import Button from '$lib/components/ui/button/button.svelte'
    import {
        CHAMPIONS_BY_ID,
        CHAMPION_ICONS,
        type CDragonChampion
    } from '$lib/constants'
    import { zip } from 'radash'
    import { derived } from 'svelte/store'
    import { getFilterFormContext } from '../../form-context/context'
    import SlotTypeInput from '../slot-type-input.svelte'

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

        <fieldset>
            <legend class="pb-1">Champions</legend>

            <p class="text-muted-foreground text-xs pb-3">
                Select champions to allow for this slot.
            </p>

            <div class="champion-grid">
                {#each zipped as [val, ctrl, c]}
                    <ChampionCheckbox
                        on:click={() => handleClick(val, ctrl)}
                        src={CHAMPION_ICONS[val.id]}
                        label={c.display_name}
                        state={val.included ? 'included' : null}
                        cost={c.tier}
                    />
                {/each}
            </div>
        </fieldset>
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
