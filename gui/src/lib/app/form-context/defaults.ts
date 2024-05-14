import { CHAMPIONS, TRAITS } from '$lib/constants'
import { deepCopy } from '$lib/utils/misc'
import { range } from 'radash'
import type {
    AttributeFilter,
    ChampionFilter,
    FilterForm,
    GlobalFilter,
    InputParser,
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

export const DEFAULT_CHAMPION_FILTER = {
    champions: CHAMPIONS.map((c) => ({
        id: c.character_id,
        included: false
    })),
    groupBy: 'cost'
} satisfies ChampionFilter

export const DEFAULT_SLOT_FILTER = {
    useAttributes: true,
    byAttribute: DEFAULT_ATTRIBUTE_FILTER,
    byChampion: DEFAULT_CHAMPION_FILTER
} satisfies SlotFilter

export const MAX_TEAM_SIZE = 11
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
    resultCount: 100,
    global: deepCopy(DEFAULT_GLOBAL_FILTER),
    slots: [...range(1, MAX_TEAM_SIZE)].map(() =>
        deepCopy(DEFAULT_SLOT_FILTER)
    )
} as const satisfies FilterForm

export const FILTER_FORM_PARSERS = {
    teamSize: IntParser,
    resultCount: IntParser,
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
        byChampion: {
            champions: { id: StringParser, included: BoolParser },
            groupBy: StringParser as
                | InputParser<'cost'>
                | InputParser<'trait'>
        }
    }
} satisfies FormParsers<FilterForm>
