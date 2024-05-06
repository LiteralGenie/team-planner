<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type { TraitFilter } from '$lib/app/form-context/types'
    import { TRAITS_BY_ID, TRAIT_ICONS } from '$lib/constants'
    import type { CDragonTrait } from '$lib/types'
    import { zip } from 'radash'
    import TraitCheckbox from '../slot-filter-form/trait-checkbox.svelte'

    const { form, controls } = getFilterFormContext()

    $: traitValues = $form.global.traits
    $: traitControls = controls.global.controls.traits.controls
    $: zipped = zip(traitValues, traitControls).map(([val, ctrl]) => [
        val,
        ctrl,
        TRAITS_BY_ID[val.id]
    ]) as Array<
        [TraitFilter, FormControlRecord<TraitFilter>, CDragonTrait]
    >

    function handleClick(
        current: TraitFilter,
        ctrl: FormControlRecord<TraitFilter>
    ) {
        ctrl.onChange({
            ...current,
            included: !current.included
        })
    }
</script>

<fieldset>
    <legend class="pb-1">Banned Traits</legend>

    <p class="text-muted-foreground text-xs pb-2">
        Select traits to exclude from results
    </p>

    <div class="trait-grid">
        {#each zipped as [val, ctrl, trait]}
            <TraitCheckbox
                on:click={() => handleClick(val, ctrl)}
                src={TRAIT_ICONS[val.id]}
                label={trait.display_name}
                state={val.included ? null : 'excluded'}
            />
        {/each}
    </div>
</fieldset>

<style lang="postcss">
    /* @todo how to make all rows the same height when one row has text wrap */
    .trait-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
        align-items: start;
        gap: 8px;
    }
</style>
