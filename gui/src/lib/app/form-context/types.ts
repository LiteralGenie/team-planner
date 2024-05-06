import type { FormControl } from './form-control'
import type { FormControlArray } from './form-control-array'
import type { FormControlRecord } from './form-control-record'

export interface FormInput {
    value: string
    addEventListener: (
        eventName: 'change',
        caldlback: () => void
    ) => void
    removeEventListener: (
        eventName: 'change',
        callback: () => void
    ) => void
}

export interface ControlLike<T> {
    setValue: (val: T) => void
    destroy: () => void
}

export interface InputParser<T> {
    fromString: (x: string) => T
    toString: (x: T) => string
}

export type Primitive = number | string | boolean | Boolean

/**
 * FormParsers<Primitive> = InputParser<Primitive>
 * FormParsers<Primitive[]> = InputParser<Primitive>
 * FormParsers<T extends Object> = { [k in keyof T]: FormParsers<T> }
 */

export type FormParsers<T> = T extends Primitive
    ? InputParser<T>
    : T extends Array<infer V>
      ? FormParsers<V>
      : {
            [K in keyof T]: FormParsers<T[K]>
        }
/**
 * Wrapper<Primitive> = FormControl<Primitive>
 * Wrapper<Primitive[]> = FormControlArray<Primitive>
 * Wrapper<T extends Object> = { [k in keyof T]: Wrapper<T> }
 */

export type FormControlWrapper<T> = T extends Primitive
    ? FormControl<T>
    : T extends Array<infer V>
      ? FormControlArray<V>
      : T extends Object
        ? FormControlRecord<T>
        : never

/****/

export interface IdFilter {
    id: string
    included: boolean
}

export type CostTier = 1 | 2 | 3 | 4 | 5

export type RangeType = 'close' | 'mid' | 'long'

export type DamageType = 'ad' | 'ap'

export interface AttributeFilter {
    cost: { [k in CostTier]: Boolean }
    range: { [k in RangeType]: Boolean }
    damageType: { [k in DamageType]: Boolean }
    traits: IdFilter[]
}

export interface SlotFilter {
    useAttributes: boolean
    byAttribute: AttributeFilter
    byId: IdFilter[]
}

export interface GlobalFilter {
    champions: IdFilter[]
    cost: { [k in CostTier]: Boolean }
    traits: IdFilter[]
}

export interface FilterForm {
    teamSize: number
    global: GlobalFilter
    slots: SlotFilter[]
}

export type FilterFormControls = {
    [K in keyof FilterForm]: FormControlWrapper<FilterForm[K]>
}

export type AttributeSlotControls = FormControlWrapper<
    FilterForm['slots'][number]['byAttribute']
>

export type AttributeSlotValues =
    FilterForm['slots'][number]['byAttribute']
