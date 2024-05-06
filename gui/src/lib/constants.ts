import CHAMPIONS from '$lib/assets/tft/tftchampions.json'
import TRAITS from '$lib/assets/tft/tfttraits.json'
import { objectify } from 'radash'

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

export const TRAITS_BY_ID = objectify(
    TRAITS,
    (t) => t.trait_id,
    (t) => t
)

const champion_icon_files = import.meta.glob(
    '$lib/assets/tft/champions/*.png',
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

export const CHAMPIONS_BY_ID = objectify(
    CHAMPIONS,
    (c) => c.character_record.character_id,
    (c) => c
)
