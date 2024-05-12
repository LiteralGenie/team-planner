import {
    AD_ICON,
    AP_ICON,
    ARMOR_ICON,
    AS_ICON,
    CRIT_ICON,
    HEALTH_ICON,
    MANA_ICON,
    RESIST_ICON
} from '$lib/constants'

export function interpolate_tooltip_images(
    tooltipHtml: string
): string {
    let result = tooltipHtml
    result = result.replaceAll('%i:scaleAP%', img(AP_ICON))
    result = result.replaceAll('%i:scaleAD%', img(AD_ICON))
    result = result.replaceAll('%i:scaleAS%', img(AS_ICON))
    result = result.replaceAll('%i:scaleCrit%', img(CRIT_ICON))
    result = result.replaceAll('%i:scaleHealth%', img(HEALTH_ICON))
    result = result.replaceAll('%i:scaleMana%', img(MANA_ICON))
    result = result.replaceAll('%i:scaleArmor%', img(ARMOR_ICON))
    result = result.replaceAll('%i:scaleMR%', img(RESIST_ICON))
    return result

    function img(src: string) {
        return `<img class="scaling-icon" src=${src}>`
    }
}
