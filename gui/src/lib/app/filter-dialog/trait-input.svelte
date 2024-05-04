<script lang="ts" context="module">
    import TRAITS from '$lib/assets/tft/tfttraits.json'
    import { objectify, zip } from 'radash'

    const files = import.meta.glob('$lib/assets/tft/traits/*.png', {
        eager: true
    })
    const srcs: Record<string, string> = objectify(
        Array.from(Object.keys(files)),
        (path: string) => {
            path = path.split('/').slice(-1)[0]
            path = path.split('.')[0]
            return path
        },
        (path) => path
    )

    const TRAITS_BY_ID = objectify(
        TRAITS,
        (t) => t.trait_id,
        (t) => t
    )
</script>

<script lang="ts">
    import type { CDragonTrait } from '$lib/types'
    import type { FormControlRecord } from '../form-context/form-control-record'
    import type {
        AttributeSlotControls,
        AttributeSlotValues,
        TraitFilter
    } from '../form-context/types'
    import TraitCheckbox from './trait-checkbox.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    $: traitValues = slotValues.traits
    $: traitControls = slotControls.controls.traits.controls
    $: zipped = zip(traitValues, traitControls).map(([val, ctrl]) => [
        val,
        ctrl,
        TRAITS_BY_ID[val.id]
    ]) as Array<
        [TraitFilter, FormControlRecord<TraitFilter>, CDragonTrait]
    >

    function getCheckboxState(val: number) {
        switch (val) {
            case 1:
                return 'included'
            case 2:
                return 'excluded'
            default:
                return null
        }
    }

    function handleClick(
        current: TraitFilter,
        ctrl: FormControlRecord<TraitFilter>
    ) {
        let update = current.state + 1
        if (update > 2) {
            update = 0
        }
        ctrl.onChange({
            ...current,
            state: update
        })
    }
</script>

<fieldset class="mt-4">
    <legend class="pb-4">Traits</legend>

    {#each zipped as [val, ctrl, trait]}
        <TraitCheckbox
            on:click={() => handleClick(val, ctrl)}
            src={srcs[val.id]}
            label={trait.display_name}
            state={getCheckboxState(val.state)}
        />
    {/each}
</fieldset>

<style lang="postcss">
    /* @todo how to make all rows the same height when one row has text wrap */
    fieldset {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
        align-items: start;
        gap: 12px;
    }
</style>
