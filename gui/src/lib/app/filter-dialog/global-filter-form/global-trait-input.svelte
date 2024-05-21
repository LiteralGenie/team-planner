<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type { IdFilter } from '$lib/app/form-context/types'
    import TraitCheckbox from '$lib/components/trait-checkbox.svelte'
    import { zip } from 'radash'

    const { form, controls } = getFilterFormContext()

    $: traitValues = $form.global.traits
    $: traitControls = controls.global.controls.traits.controls
    $: zipped = zip(traitValues, traitControls)

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
    <legend class="pb-1">Banned Traits</legend>

    <p class="text-muted-foreground text-xs pb-2">
        Teams containing any of the selected traits will not be
        displayed.
    </p>

    <div class="trait-grid">
        {#each zipped as [val, ctrl]}
            <TraitCheckbox
                on:click={() => handleClick(val, ctrl)}
                id={val.id}
                value={val.included ? null : 'excluded'}
                portal="dialog"
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
