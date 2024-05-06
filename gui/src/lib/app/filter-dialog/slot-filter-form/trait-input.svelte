<script lang="ts">
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type {
        AttributeSlotControls,
        AttributeSlotValues,
        IdFilter
    } from '$lib/app/form-context/types'
    import TraitCheckbox from '$lib/components/trait-checkbox.svelte'
    import type { CDragonTrait } from '$lib/constants'
    import { TRAITS_BY_ID, TRAIT_ICONS } from '$lib/constants'
    import { zip } from 'radash'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    $: traitValues = slotValues.traits
    $: traitControls = slotControls.controls.traits.controls
    $: zipped = zip(traitValues, traitControls).map(([val, ctrl]) => [
        val,
        ctrl,
        TRAITS_BY_ID[val.id]
    ]) as Array<[IdFilter, FormControlRecord<IdFilter>, CDragonTrait]>

    function handleClick(
        current: IdFilter,
        ctrl: FormControlRecord<IdFilter>
    ) {
        ctrl.onChange({
            ...current,
            included: !current.included
        })
    }
</script>

<fieldset>
    <legend class="pb-2">Traits</legend>

    {#each zipped as [val, ctrl, trait]}
        <TraitCheckbox
            on:click={() => handleClick(val, ctrl)}
            src={TRAIT_ICONS[val.id]}
            label={trait.display_name}
            state={val.included ? 'included' : null}
        />
    {/each}
</fieldset>

<style lang="postcss">
    /* @todo how to make all rows the same height when one row has text wrap */
    fieldset {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
        align-items: start;
        gap: 8px;
    }
</style>
