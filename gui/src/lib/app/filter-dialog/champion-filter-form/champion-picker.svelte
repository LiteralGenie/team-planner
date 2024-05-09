<script lang="ts" context="module">
    import {
        CHAMPIONS,
        CHAMPION_ICONS,
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
</script>

<script lang="ts">
    import type { FormControlRecord } from '$lib/app/form-context/form-control-record'
    import type { IdFilter } from '$lib/app/form-context/types'
    import ChampionCheckbox from '$lib/components/champion-checkbox.svelte'
    import { alphabetical, group, sort } from 'radash'
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

            const control = $slotControls[idx]

            acc[filter.id] = {
                id: filter.id,
                control,
                included: filter.included,
                disabled: !globalFilter.included
            }

            return acc
        },
        {} as Record<string, SelectionState>
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
</script>

<fieldset>
    <div class="pb-5 flex items-center justify-between">
        <div class="flex flex-col justify-center">
            <legend class="pb-1">Champions</legend>
            <p class="text-muted-foreground text-xs">
                Select champions to allow for this slot.
            </p>
        </div>

        <select class="">
            <option>a dsafs </option>
            <option>afds</option>
            <option>afsdaf</option>
            <option>a fsdfa sd</option>
        </select>
    </div>

    <div>
        {#each getGroups('trait') as group}
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
                        <ChampionCheckbox
                            on:click={() =>
                                handleClick(c.character_id)}
                            src={CHAMPION_ICONS[c.character_id]}
                            label={c.display_name}
                            value={states[c.character_id].included
                                ? 'included'
                                : null}
                            cost={c.tier}
                            disabled={states[c.character_id].disabled}
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

    select {
        @apply rounded-md;

        background-color: hsl(var(--background));
        color: hsl(var(--foreground));
        padding: 0.25em 0.5em;
    }
</style>
