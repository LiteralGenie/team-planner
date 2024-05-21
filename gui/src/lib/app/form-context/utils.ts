import {
    CHAMPIONS,
    CHAMPIONS_BY_ID,
    TRAITS_BY_ID,
    type CDragonTrait,
    type CDragonTraitLevel
} from '$lib/constants'
import { deepCopy, filterMap, someFalse } from '$lib/utils/misc'
import { isArray, isObject } from 'radash'
import { FormControl } from './form-control'
import { FormControlArray } from './form-control-array'
import { FormControlRecord } from './form-control-record'
import type {
    AttributeFilter,
    CostTier,
    DamageType,
    FilterForm,
    FormControlWrapper,
    GlobalFilter,
    IdFilter,
    InputParser,
    RangeType,
    SlotFilter
} from './types'

export const StringParser = {
    fromString: (val: string) => val,
    toString: (val: string) => val
}
export const IntParser = {
    fromString: (val: string) => parseInt(val),
    toString: (val: number) => String(val)
}

// @todo: Not sure why we have to cast this to any
//        maybe related: https://github.com/microsoft/TypeScript/issues/24413
export const BoolParser = {
    fromString: (val: string) => Boolean(val),
    toString: (val: boolean) => String(val)
} as InputParser<any>

export type ValueOf<T> = T[keyof T]

export function createControl<T>(
    initValue: T,
    // @fixme: any
    parser: any,
    onChange: (val: T) => void
): FormControlWrapper<T> {
    if (isArray(initValue)) {
        // @ts-ignore
        return new FormControlArray(onChange, parser, initValue)
    } else if (isObject(initValue)) {
        // @ts-ignore
        return new FormControlRecord(onChange, parser, initValue)
    } else {
        // @ts-ignore
        return new FormControl(onChange, parser)
    }
}

export interface ActiveFilters {
    cost?: CostTier[]
    range?: RangeType[]
    damageType?: DamageType[]
    traits?: IdFilter[]
    champions?: IdFilter[]
}

export function getActiveSlotFilters(
    global: GlobalFilter,
    slot: SlotFilter
): ActiveFilters {
    let activeFilters: ActiveFilters = {}

    if (slot.useAttributes) {
        const attrFilters = slot.byAttribute

        // If some cost filters disabled (excluding those disabled by global filter), mark cost as active filter
        const costFilters = Object.entries(attrFilters.cost).filter(
            ([cost, _]) => global.cost[cost as any as CostTier]
        ) as any as Array<[CostTier, boolean]>
        if (costFilters.some(([c, included]) => !included)) {
            activeFilters.cost = filterMap(
                costFilters,
                ([cost, included]) => (included ? cost : null)
            )
        }

        if (someFalse(attrFilters.range)) {
            activeFilters.range = filterMap<RangeType, any>(
                Object.entries(attrFilters.range),
                ([range, val]) => (val ? range : null)
            )
        }

        if (someFalse(attrFilters.damageType)) {
            activeFilters.damageType = filterMap<DamageType, any>(
                Object.entries(attrFilters.damageType),
                ([damageType, val]) => (val ? damageType : null)
            )
        }

        const traitsSelected = attrFilters.traits.filter((t) => {
            const globalTrait = global.traits.find(
                ({ id }) => id === t.id
            )
            return t.included && globalTrait?.included
        })
        if (traitsSelected.length > 0) {
            activeFilters.traits = traitsSelected
        }
    } else {
        const activeIds = slot.byChampion.champions.filter((c) => {
            const globalChampion = global.champions.find(
                ({ id }) => id === c.id
            )!

            return c.included && globalChampion.included
        })

        if (activeIds.length) {
            activeFilters.champions = deepCopy(activeIds)
        }
    }

    return activeFilters
}

export function applyGlobalFilter(filter: GlobalFilter) {
    const activeTraits = new Set(
        filter.traits
            .filter(({ included }) => included)
            .map(({ id }) => id)
    )

    const activeCostTiers = new Set(
        Object.entries(filter.cost)
            .filter(([_, included]) => included)
            .map(([cost, _]) => parseInt(cost) as CostTier)
    )

    const activeChampions = new Set(
        filter.champions.filter((c) => c.included).map((c) => c.id)
    )

    return new Set(
        CHAMPIONS.filter((c) =>
            c.traits.every(
                (t) => activeTraits.has(t.id) || !TRAITS_BY_ID[t.id]
            )
        )
            .filter((c) => activeCostTiers.has(c.tier))
            .filter((c) => activeChampions.has(c.character_id))
            .map((c) => c.character_id)
    )
}

export function applyAttributeFilter(
    filter: AttributeFilter
): Set<String> {
    const activeTraits = new Set(
        filter.traits.filter((t) => t.included).map((t) => t.id)
    )

    return new Set(
        CHAMPIONS.filter((c) => filter.cost[c.tier])
            .filter(
                (c) =>
                    filter.range[mapRangeValueToType(c.stats.range)]
            )
            .filter((c) => {
                if (filter.damageType.ad && filter.damageType.ap) {
                    return true
                } else if (filter.damageType.ad) {
                    return c.damage_type.is_ad
                } else if (filter.damageType.ap) {
                    return c.damage_type.is_ap
                } else {
                    // User unchecked both damage type filters for whatever reason
                    return false
                }
            })
            .filter((c) => {
                const isEmpty = activeTraits.size === 0
                const isActive = c.traits.some((t) =>
                    activeTraits.has(t.id)
                )
                return isEmpty || isActive
            })
            .map((c) => c.character_id)
    )
}

export function applyAttributeFilterWithGlobal(
    globalFilter: GlobalFilter,
    attrFilter: AttributeFilter
): Set<string> {
    const byGlobal = applyGlobalFilter(globalFilter)

    // Override attribute filters that conflict with globals
    attrFilter = deepCopy(attrFilter)
    Object.values(globalFilter.traits)
        .filter(({ included }) => !included)
        .forEach(({ id }) => {
            const trait = attrFilter.traits.find((t) => id === t.id)!
            trait.included = false
        })

    const byAttr = applyAttributeFilter(attrFilter)

    // Return intersection of sets
    return new Set(
        Array.from(byGlobal.values()).filter((c) => byAttr.has(c))
    )
}

export function mapRangeValueToType(value: number): RangeType {
    if (value <= 1) {
        return 'close'
    } else if (value <= 2) {
        return 'mid'
    } else {
        return 'long'
    }
}

export function serializeFilterForm(form: FilterForm): string {
    return 'lmoa'
}

export type TraitLevel = CDragonTraitLevel & {
    levelIdx: number
}

export function getTraitLevel(
    unitCount: number,
    trait: CDragonTrait
): TraitLevel | null {
    let traitLevel: TraitLevel | null = null
    for (let [levelIdx, level] of trait.levels.entries()) {
        if (unitCount >= level.min_units) {
            traitLevel = { ...level, levelIdx }
        } else {
            break
        }
    }

    return traitLevel
}

export function tallyTraits(
    champions: string[]
): Record<string, number> {
    return champions
        .map((id) => CHAMPIONS_BY_ID[id])
        .reduce(
            (acc, c) => {
                for (let trait of c.traits) {
                    if (!TRAITS_BY_ID[trait.id]) {
                        continue
                    }

                    acc[trait.id] = acc[trait.id] || 0
                    acc[trait.id] += 1
                }

                return acc
            },
            {} as Record<string, number>
        )
}
