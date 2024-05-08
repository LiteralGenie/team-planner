import { CHAMPIONS, TRAITS } from '$lib/constants'
import { deepCopy } from '$lib/utils/misc'
import { range } from 'radash'
import type {
    AttributeFilter,
    FilterForm,
    GlobalFilter,
    SlotFilter
} from './types'
import { type FormParsers } from './types'
import { BoolParser, IntParser, StringParser } from './utils'

const DEFAULT_ATTRIBUTE_FILTER = {
    cost: {
        1: true,
        2: true,
        3: true,
        4: true,
        5: true
    },
    range: {
        close: true,
        mid: true,
        long: true
    },
    traits: TRAITS.map((t) => ({
        id: t.trait_id,
        included: false
    })),
    damageType: {
        ad: true,
        ap: true
    }
} satisfies AttributeFilter

export const DEFAULT_SLOT_FILTER = {
    useAttributes: true,
    byAttribute: DEFAULT_ATTRIBUTE_FILTER,
    byId: CHAMPIONS.map((c) => ({
        id: c.character_id,
        included: false
    }))
} satisfies SlotFilter

const DEFAULT_TEAM_SIZE = 7

export const DEFAULT_GLOBAL_FILTER = {
    champions: CHAMPIONS.map((c) => ({
        id: c.character_id,
        included: true
    })),
    cost: {
        1: true,
        2: true,
        3: true,
        4: true,
        5: true
    },
    traits: TRAITS.map((t) => ({
        id: t.trait_id,
        included: true
    }))
} satisfies GlobalFilter

export const DEFAULT_FILTER_FORM = {
    teamSize: DEFAULT_TEAM_SIZE,
    global: deepCopy(DEFAULT_GLOBAL_FILTER),
    slots: [...range(1, DEFAULT_TEAM_SIZE)].map((_) =>
        deepCopy(DEFAULT_SLOT_FILTER)
    )
} as const satisfies FilterForm

export const FILTER_FORM_PARSERS = {
    teamSize: IntParser,
    global: {
        cost: {
            1: BoolParser,
            2: BoolParser,
            3: BoolParser,
            4: BoolParser,
            5: BoolParser
        },
        traits: {
            id: StringParser,
            included: BoolParser
        },
        champions: {
            id: StringParser,
            included: BoolParser
        }
    },
    slots: {
        useAttributes: BoolParser,
        byAttribute: {
            cost: {
                1: BoolParser,
                2: BoolParser,
                3: BoolParser,
                4: BoolParser,
                5: BoolParser
            },
            range: {
                close: BoolParser,
                mid: BoolParser,
                long: BoolParser
            },
            traits: {
                id: StringParser,
                included: BoolParser
            },
            damageType: {
                ad: BoolParser,
                ap: BoolParser
            }
        },
        byId: {
            id: StringParser,
            included: BoolParser
        }
    }
} satisfies FormParsers<FilterForm>
