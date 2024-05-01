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

type Primitive = number | string | boolean

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

export const StringParser = {
    fromString: (val: string) => val,
    toString: (val: string) => val
}
export const IntParser = {
    fromString: (val: string) => parseInt(val),
    toString: (val: number) => String(val)
}
export const BoolParser = {
    fromString: (val: string) => Boolean(val),
    toString: (val: boolean) => String(val)
}

export type ValueOf<T> = T[keyof T]
