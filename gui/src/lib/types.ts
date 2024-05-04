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
