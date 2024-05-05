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
    traits: [],
    damageType: {
        ad: true,
        ap: true
    }
} satisfies AttributeFilter

const DEFAULT_SLOT_FILTER = {
    useAttributes: true,
    byAttribute: DEFAULT_ATTRIBUTE_FILTER,
    byId: []
} satisfies SlotFilter

const DEFAULT_TEAM_SIZE = 7

const DEFAULT_GLOBAL_FILTER = {
    champions: [],
    cost: {
        1: true,
        2: true,
        3: true,
        4: true,
        5: true
    },
    traits: []
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
        champions: StringParser
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
        byId: StringParser
    }
} satisfies FormParsers<FilterForm>
