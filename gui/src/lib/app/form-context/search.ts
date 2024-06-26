import type { FilterForm } from '$lib/app/form-context/types'
import { applyAttributeFilterWithGlobal } from '$lib/app/form-context/utils'
import { CHAMPIONS_BY_ID } from '$lib/constants'
import { invert, range } from 'radash'

const worker = new Worker('worker.js')

// Worker's init function is async
const workerInit = new Promise((resolve) =>
    worker.addEventListener('message', (ev) => {
        if (ev.data.type === 'ready') {
            resolve(true)
        }
    })
)

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

export async function setSearchOptions(form: FilterForm) {
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

    const var_to_champion = invert(champion_to_var)

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

    await workerInit

    worker.postMessage({
        type: 'setOptions',
        options,
        var_to_champion
    })
}

export async function getSearchResult(
    batchSize: number
): Promise<string[][]> {
    const start = Date.now()

    worker.postMessage({
        type: 'nextSolution',
        batchSize
    })

    return new Promise((resolve) => {
        worker.onmessage = (ev) => {
            console.log(
                `Fetched ${batchSize} results in ${Math.floor(Date.now() - start)}ms`
            )

            resolve(ev.data)
        }
    })
}
