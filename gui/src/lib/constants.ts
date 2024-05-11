import type { CostTier } from '$lib/app/form-context/types'
import ALL_CHAMPIONS from '$lib/assets/tft/merged_teamplanner_data.json'
import ap_icon from '$lib/assets/tft/misc/statmodsabilitypowericon.png'
import ad_icon from '$lib/assets/tft/misc/statmodsattackdamageicon.png'
import ALL_TRAITS from '$lib/assets/tft/tfttraits.json'
import { alphabetical, group, objectify, sort } from 'radash'

export const AP_ICON = ap_icon
export const AD_ICON = ad_icon

export const TRAITS = ALL_TRAITS.filter(
    (t) => t.display_name !== 'Exalted'
)
export const CHAMPIONS = sortChampions(
    ALL_CHAMPIONS.map((c) => ({ ...c, tier: c.tier as CostTier }))
)

function sortChampions(champions: CDragonChampion[]) {
    const tiersSorted = sort(
        Object.values(group(champions, (c) => c.tier)),
        (cs) => cs[0].tier
    )

    return tiersSorted
        .map((cs) => alphabetical(cs, (c) => c.display_name))
        .flatMap((cs) => cs)
}

const trait_icon_files = import.meta.glob(
    '$lib/assets/tft/traits/*.png',
    {
        eager: true
    }
)

export const TRAIT_ICONS: Record<string, string> = objectify(
    Array.from(Object.keys(trait_icon_files)),
    // Trait ID as key (ie filename minus extension)
    (path: string) => {
        const name = path.split('/').slice(-1)[0]
        const stem = name.split('.')[0]
        return stem
    },
    // Path as value
    (path) => path
)

const ability_icon_files = import.meta.glob(
    '$lib/assets/tft/abilities/*.png',
    {
        eager: true
    }
)

export const ABILITY_ICONS: Record<string, string> = objectify(
    Array.from(Object.keys(ability_icon_files)),
    // Trait ID as key (ie filename minus extension)
    (path: string) => {
        const name = path.split('/').slice(-1)[0]
        const stem = name.split('.')[0]
        return stem
    },
    // Path as value
    (path) => path
)

export const TRAITS_BY_ID = objectify(
    TRAITS,
    (t) => t.trait_id,
    (t) => t
) satisfies Record<string, CDragonTrait>

const champion_icon_files = import.meta.glob(
    '$lib/assets/tft/champions/*',
    {
        eager: true
    }
)
export const CHAMPION_ICONS: Record<string, string> = objectify(
    Array.from(Object.keys(champion_icon_files)),
    // Trait ID as key (ie filename minus extension)
    (path: string) => {
        const name = path.split('/').slice(-1)[0]
        const stem = name.split('.')[0]
        return stem
    },
    // Path as value
    (path) => path
)

const champion_splash_files = import.meta.glob(
    '$lib/assets/tft/champion_splashes/*',
    {
        eager: true
    }
)
export const CHAMPION_SPLASHES: Record<string, string> = objectify(
    Array.from(Object.keys(champion_splash_files)),
    // Trait ID as key (ie filename minus extension)
    (path: string) => {
        const name = path.split('/').slice(-1)[0]
        const stem = name.split('.')[0]
        return stem
    },
    // Path as value
    (path) => path
)

export const CHAMPIONS_BY_ID = objectify(
    CHAMPIONS,
    (c) => c.character_id,
    (c) => c
) satisfies Record<string, CDragonChampion>

export interface CDragonTrait {
    display_name: string
    trait_id: string
    set: string
    tooltip_text: string
    innate_trait_sets: Array<{
        effect_amounts: Array<{
            name: string
            value: number
            format_string: string
        }>
    }>
    conditional_trait_sets: Array<{
        effect_amounts: Array<{
            name: string
            value: number
            format_string: string
        }>
        min_units: number
        max_units?: number
        style_idx: number
        style_name: string
    }>
}

export interface CDragonChampion {
    character_id: string
    tier: CostTier
    display_name: string
    traits: Array<{
        name: string
        id: string
        amount: number
    }>
    damage_type: {
        is_ad: boolean
        is_ap: boolean
    }
    stats: {}
    spell: string
}
