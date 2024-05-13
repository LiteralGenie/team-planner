import type { FilterForm } from '$lib/app/form-context/types'
import { applyAttributeFilterWithGlobal } from '$lib/app/form-context/utils'
import { CHAMPIONS_BY_ID } from '$lib/constants'
import { invert, range } from 'radash'

function getSlotOptions(form: FilterForm): Array<string[]> {
    const slots: Array<string[]> = []

    const globalWhitelist = new Set(
        form.global.champions
            .filter(({ included }) => included)
            .map(({ id }) => id)
    )

    for (let idx of range(0, form.teamSize - 1)) {
        const slot = form.slots[idx]

        if (slot.useAttributes) {
            const champions = applyAttributeFilterWithGlobal(
                form.global,
                slot.byAttribute
            )
            slots.push([...champions.values()])
        } else {
            const champions = slot.byChampion.champions
                .filter(({ included }) => included)
                .filter(({ id }) => globalWhitelist.has(id))
                .map(({ id }) => id)

            slots.push(champions)
        }
    }

    return slots
}

export function doSearch(form: FilterForm): string[][] {
    const slotOptions = getSlotOptions(form)

    const allChampions: Set<string> = new Set()
    for (let opts of slotOptions) {
        for (let c of opts) {
            allChampions.add(c)
        }
    }

    const champion_to_var = [...allChampions.values()].reduce(
        (acc, id, idx) => {
            // Filtering champs globally banned reduces calculation time
            // @todo: this should really be done on rust side
            if (allChampions.has(id)) {
                acc[id] = idx
            }

            return acc
        },
        {} as Record<string, number>
    )

    const slots = slotOptions.map((cs) =>
        cs.map((c) => champion_to_var[c])
    )

    const traits = new Map()
    ;[...allChampions.values()]
        .map((id) => CHAMPIONS_BY_ID[id])
        .map((c) => [
            champion_to_var[c.character_id],
            c.traits.map((t) => t.name)
        ])
        .forEach(([k, v]) => traits.set(k as number, v as string[]))

    const num_champions = Object.values(champion_to_var).length

    const options = {
        num_champions,
        team_size: form.teamSize,
        slots,
        traits
    }

    console.log('Searching with options', options)
    const teams = window.tft.search_teams(options)

    const var_to_champion = invert(champion_to_var)
    const remapped = teams.map(({ champion_ids }) =>
        champion_ids.map((v) => var_to_champion[v])
    )

    return remapped
}
