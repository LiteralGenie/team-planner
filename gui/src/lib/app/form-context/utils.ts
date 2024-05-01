import { isArray, isObject } from 'radash'
import { FormControl } from './form-control'
import { FormControlArray } from './form-control-array'
import { FormControlGroup } from './form-control-group'

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

// @todo: rename to differentiate FormControlsContainer and FormControlsGroup
//        maybe latter to FormControlsRecord and former to Group
/**
 * Container<Primitive> = FormControl<Primitive>
 * Container<Primitive[]> = FormControlArray<Primitive>
 * Container<T extends Object> = { [k in keyof T]: Container<T> }
 */
export type FormControlsContainer<T> = T extends Primitive
    ? FormControl<T>
    : T extends Array<infer V>
      ? FormControlArray<V>
      : T extends Object
        ? FormControlGroup<T>
        : never

export const StringParser = {
    fromString: (val: string) => val,
    toString: (val: string) => val
}
export const IntParser = {
    fromString: (val: string) => parseInt(val),
    toString: (val: number) => String(val)
}

// @todo: Not sure why we have to cast this to any
//        maybe related: https://github.com/microsoft/TypeScript/issues/24413
export const BoolParser = {
    fromString: (val: string) => Boolean(val),
    toString: (val: boolean) => String(val)
} as InputParser<any>

export type ValueOf<T> = T[keyof T]

export function createControl<T>(
    initValue: T,
    // @fixme: any
    parser: any,
    onChange: (val: T) => void
): FormControlsContainer<T> {
    if (isArray(initValue)) {
        // @ts-ignore
        return new FormControlArray(onChange, parser, initValue)
    } else if (isObject(initValue)) {
        // @ts-ignore
        return new FormControlGroup(onChange, parser, initValue)
    } else {
        // @ts-ignore
        return new FormControl(onChange, parser)
    }
}
