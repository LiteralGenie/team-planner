import TRAITS from '$lib/assets/tft/tfttraits.json'
import { objectify } from 'radash'

const files = import.meta.glob('$lib/assets/tft/traits/*.png', {
    eager: true
})
export const TRAIT_ICONS: Record<string, string> = objectify(
    Array.from(Object.keys(files)),
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
