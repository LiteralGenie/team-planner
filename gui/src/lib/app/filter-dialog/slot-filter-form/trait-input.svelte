<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type {
        AttributeSlotControls,
        AttributeSlotValues,
        IdFilter
    } from '$lib/app/form-context/types'
    import TraitCheckbox from '$lib/components/trait-checkbox.svelte'
    import TraitTooltip from '$lib/components/trait-tooltip.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import type { CDragonTrait } from '$lib/constants'
    import { TRAITS_BY_ID, TRAIT_ICONS } from '$lib/constants'
    import { zip } from 'radash'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    const { form } = getFilterFormContext()

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

    function isDisabledGlobally(id_trait: string) {
        const trait = $form.global.traits.find(
            ({ id }) => id === id_trait
        )!
        return !trait.included
    }
</script>

<fieldset>
    <legend class="pb-2">Traits</legend>

    {#each zipped as [val, ctrl, trait]}
        <Tooltip.Root
            openDelay={100}
            closeOnPointerDown={true}
            portal="dialog"
            group="trait"
        >
            <Tooltip.Trigger>
                <TraitCheckbox
                    on:click={() => handleClick(val, ctrl)}
                    src={TRAIT_ICONS[val.id]}
                    label={trait.display_name}
                    value={val.included ? 'included' : null}
                    disabled={isDisabledGlobally(val.id)}
                    disabledTooltip="Disabled by global filter"
                />
            </Tooltip.Trigger>
            <Tooltip.Content class="boring-tooltip max-w-[400px]">
                <TraitTooltip trait_id={val.id} />
            </Tooltip.Content>
        </Tooltip.Root>
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
