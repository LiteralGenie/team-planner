import { CHAMPIONS } from '$lib/constants'
import { deepCopy, filterMap, someFalse } from '$lib/utils/misc'
import { isArray, isObject } from 'radash'
import { FormControl } from './form-control'
import { FormControlArray } from './form-control-array'
import { FormControlRecord } from './form-control-record'
import type {
    AttributeFilter,
    CostTier,
    DamageType,
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

        const traitsSelected = attrFilters.traits.filter(
            (t) => t.included
        )
        if (traitsSelected.length > 0) {
            activeFilters.traits = traitsSelected
        }
    } else {
        const activeIds = slot.byId.filter(({ included }) => included)

        if (activeIds.length) {
            activeFilters.champions = deepCopy(activeIds)
        }
    }

    return activeFilters
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
            .filter(
                (c) =>
                    activeTraits.size === 0 ||
                    c.traits.some((t) => activeTraits.has(t.id))
            )
            .map((c) => c.character_id)
    )
}

export function mapRangeValueToType(value: number): RangeType {
    if (value <= 1.0) {
        return 'close'
    } else if (value < 3) {
        return 'mid'
    } else {
        return 'long'
    }
}
