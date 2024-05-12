<script lang="ts" context="module">
    import {
        CHAMPIONS,
        TRAITS_BY_ID,
        type CDragonChampion,
        type CDragonTrait
    } from '$lib/constants'

    interface ChampionGroup<T = any> {
        criteria: T
        champions: CDragonChampion[]
        label?: string
        prefix?: {
            component: typeof SvelteComponent<any>
            props: any
        }
    }

    interface SelectionState {
        id: string
        control: FormControlRecord<IdFilter>
        disabled: boolean
        included: boolean
    }

    const groupByOptions = [
        { value: 'cost', text: 'Group by cost' },
        { value: 'trait', text: 'Group by trait' }
    ]
</script>

<script lang="ts">
    import type { FormControl } from '$lib/app/form-context/form-control'
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type {
        ChampionGroupingType,
        IdFilter
    } from '$lib/app/form-context/types'
    import ChampionCheckbox from '$lib/components/champion-checkbox.svelte'
    import { alphabetical, group, sort } from 'radash'
    import Svelecte from 'svelecte'
    import type { SvelteComponent } from 'svelte'
    import { derived } from 'svelte/store'
    import { getFilterFormContext } from '../../form-context/context'
    import PickerCostIcon from './picker-cost-icon.svelte'
    import PickerTraitIcon from './picker-trait-icon.svelte'

    export let slotIndex: number

    const { form, controls } = getFilterFormContext()
    $: slotValues = $form.slots[slotIndex].byChampion.champions
    $: slotControls = derived(
        controls.slots.controlsStore,
        (controls) =>
            controls[slotIndex].controls.byChampion.controls.champions
                .controls
    )

    $: states = slotValues.reduce(
        (acc, filter, idx) => {
            const globalFilter = $form.global.champions.find(
                (c) => c.id === filter.id
            )!

            const disabled = !globalFilter.included

            const control = $slotControls[idx]

            acc[filter.id] = {
                id: filter.id,
                control,
                included: filter.included,
                disabled
            }

            return acc
        },
        {} as Record<string, SelectionState>
    )

    $: groupByValue = $form.slots[slotIndex].byChampion.groupBy
    $: groupByControl = derived(
        controls.slots.controlsStore,
        (controls) =>
            controls[slotIndex].controls.byChampion.controls.groupBy
    )

    function getGroups(criteria: 'cost' | 'trait'): ChampionGroup[] {
        if (criteria === 'cost') {
            let groups = Object.values(
                group(CHAMPIONS, (c) => c.tier)
            ).map((cs) => {
                const tier = cs[0].tier
                return {
                    criteria: tier,
                    champions: cs,
                    prefix: {
                        component: PickerCostIcon,
                        props: { cost: tier }
                    }
                }
            })

            return sort(groups, (g) => g.criteria)
        } else {
            const byTrait: Record<
                string,
                ChampionGroup<CDragonTrait>
            > = {}
            for (let c of CHAMPIONS) {
                for (let t of c.traits) {
                    if (!byTrait[t.id]) {
                        const info = TRAITS_BY_ID[t.id]

                        if (!info) {
                            // This is a unique trait and downloader script skipped it
                            continue
                        }

                        byTrait[t.id] = {
                            criteria: TRAITS_BY_ID[t.id],
                            champions: [],
                            prefix: {
                                component: PickerTraitIcon,
                                props: {
                                    id: t.id
                                }
                            }
                        }
                    }

                    byTrait[t.id].champions.push(c)
                }
            }

            let groups = Object.values(byTrait)
            return alphabetical(
                groups,
                (g) => g.criteria.display_name
            )
        }
    }

    function handleClick(id: string) {
        const { control, included } = states[id]

        control.onChange({
            id,
            included: !included
        })
    }

    function handleGroupBy(value: ChampionGroupingType) {
        ;($groupByControl as FormControl<string>).onChange(value)
    }
</script>

<fieldset>
    <div
        class="pb-12 sm:pb-5 flex items-center justify-between gap-4"
    >
        <div class="flex flex-col justify-center">
            <legend class="pb-1">Champions</legend>
            <p class="text-muted-foreground text-xs">
                Select champions to allow for this slot.
            </p>
        </div>

        <Svelecte
            on:change={(ev) => handleGroupBy(ev.detail.value)}
            options={groupByOptions}
            value={groupByValue}
            searchable={false}
            keepSelectionInList={true}
            highlightFirstItem={false}
            clearable={false}
            class="max-w-40"
        />
    </div>

    <div>
        {#each getGroups(groupByValue) as group}
            <div class="row flex items-center justify-center">
                <div class="h-full pr-4 flex gap-2">
                    {#if group.prefix}
                        <svelte:component
                            this={group.prefix.component}
                            {...group.prefix.props}
                        />
                    {/if}

                    {#if group.label}
                        {group.label}
                    {/if}
                </div>

                <div class="flex-grow border-l-2 champion-grid p-4">
                    {#each group.champions as c}
                        {@const state = states[c.character_id]}

                        <ChampionCheckbox
                            on:click={() =>
                                handleClick(c.character_id)}
                            id={c.character_id}
                            value={state.included ? 'included' : null}
                            disabled={state.disabled}
                            disabledTooltip="Disabled by global filter"
                        />
                    {/each}
                </div>
            </div>
        {/each}
    </div>
</fieldset>

<style lang="postcss">
    .champion-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(45px, 1fr));
        align-items: start;
        gap: 12px 12px;
    }

    .row:not(:first-child) {
        @apply border-t-2;
    }
</style>
