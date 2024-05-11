import { AD_ICON, AP_ICON } from '$lib/constants'

export function interpolate_tooltip_images(
    tooltipHtml: string
): string {
    let result = tooltipHtml
    result = result.replaceAll('%i:scaleAP%', img(AP_ICON))
    result = result.replaceAll('%i:scaleAD%', img(AD_ICON))
    return result

    function img(src: string) {
        return `<img src=${src}>`
    }
}
