<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type { IdFilter } from '$lib/app/form-context/types'
    import TraitCheckbox from '$lib/components/trait-checkbox.svelte'
    import TraitTooltip from '$lib/components/trait-tooltip.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import {
        TRAITS_BY_ID,
        TRAIT_ICONS,
        type CDragonTrait
    } from '$lib/constants'
    import { zip } from 'radash'

    const { form, controls } = getFilterFormContext()

    $: traitValues = $form.global.traits
    $: traitControls = controls.global.controls.traits.controls
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
    <legend class="pb-1">Banned Traits</legend>

    <p class="text-muted-foreground text-xs pb-2">
        Teams containing any of the selected traits will not be
        displayed.
    </p>

    <div class="trait-grid">
        {#each zipped as [val, ctrl, trait]}
            <Tooltip.Root
                openDelay={100}
                closeOnPointerDown={true}
                portal="dialog"
                group="trait"
                disableHoverableContent={true}
            >
                <Tooltip.Trigger>
                    <TraitCheckbox
                        on:click={() => handleClick(val, ctrl)}
                        src={TRAIT_ICONS[val.id]}
                        label={trait.display_name}
                        value={val.included ? null : 'excluded'}
                    />
                </Tooltip.Trigger>
                <Tooltip.Content class="boring-tooltip max-w-[400px]">
                    <TraitTooltip trait_id={val.id} />
                </Tooltip.Content>
            </Tooltip.Root>
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
