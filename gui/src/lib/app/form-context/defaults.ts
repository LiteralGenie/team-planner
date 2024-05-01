import { clone, range } from 'radash'
import type {
    AttributeFilter,
    FilterForm,
    SlotFilter
} from './context'
import { type FormParsers } from './types'
import { BoolParser, IntParser, StringParser } from './utils'

const DEFAULT_ATTRIBUTE_FILTER = {
    cost: [true, true, true, true, true],
    range: [true, true, true],
    traitIdsExcluded: [],
    damageType: [true, true]
} satisfies AttributeFilter
const DEFAULT_SLOT_FILTER = {
    useAttributes: true,
    byAttribute: DEFAULT_ATTRIBUTE_FILTER,
    byId: []
} satisfies SlotFilter
const DEFAULT_TEAM_SIZE = 7

export const DEFAULT_FILTER_FORM = {
    teamSize: DEFAULT_TEAM_SIZE,
    slots: [...range(DEFAULT_TEAM_SIZE)].map((_) =>
        clone(DEFAULT_SLOT_FILTER)
    )
} as const satisfies FilterForm

export const FILTER_FORM_PARSERS = {
    teamSize: IntParser,
    slots: {
        useAttributes: BoolParser,
        byAttribute: {
            cost: BoolParser,
            range: BoolParser,
            traitIdsExcluded: StringParser,
            damageType: BoolParser
        },
        byId: StringParser
    }
} satisfies FormParsers<FilterForm>
