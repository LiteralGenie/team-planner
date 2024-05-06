<script lang="ts">
    import { getFilterFormContext } from '$lib/app/form-context/context'
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type { IdFilter } from '$lib/app/form-context/types'
    import ChampionCheckbox from '$lib/components/champion-checkbox.svelte'
    import {
        CHAMPIONS_BY_ID,
        CHAMPION_ICONS,
        type CDragonChampion
    } from '$lib/constants'
    import { zip } from 'radash'

    const { form, controls } = getFilterFormContext()

    $: championValues = $form.global.champions
    $: championControls = controls.global.controls.champions.controls
    $: zipped = zip(championValues, championControls).map(
        ([val, ctrl]) => [val, ctrl, CHAMPIONS_BY_ID[val.id]]
    ) as Array<
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
</script>

<fieldset>
    <legend class="pb-1">Banned Champions</legend>

    <p class="text-muted-foreground text-xs pb-3">
        Teams containing any of these champions are omitted.
    </p>

    <div class="input-grid">
        {#each zipped as [val, ctrl, c]}
            <ChampionCheckbox
                on:click={() => handleClick(val, ctrl)}
                src={CHAMPION_ICONS[val.id]}
                label={c.display_name}
                state={val.included ? null : 'excluded'}
                cost={c.tier}
            />
        {/each}
    </div>
</fieldset>

<style lang="postcss">
    /* @todo how to make all rows the same height when one row has text wrap */
    .input-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
        align-items: start;
        gap: 12px 12px;
    }
</style>
