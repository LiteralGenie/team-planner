import sys
import traceback
from dataclasses import dataclass
from functools import cached_property

STAT_MAP = {
    1: dict(key="baseArmor", scaling=1),
    2: dict(key="baseDamage", scaling=1.5),
    3: dict(key="attackSpeed", scaling=1),
    5: dict(key="baseSpellBlock", scaling=1),
    11: dict(key="baseHP", scaling=1.8),
}

MAX_LEVEL = 6


@dataclass
class CalculationContext:
    mDataValues: dict
    mSpellCalculations: dict
    stats: dict

    @cached_property
    def mDataValuesMapped(self):
        # Variable casing is inconsistent...
        #   eg formulas reference AoE but correct name is AOE
        return {x["mName"].lower(): x for x in self.mDataValues}


def compute_spell_variables(
    mSpell: dict,
    stats: dict,
) -> dict[str, list[float]]:
    ctx = CalculationContext(
        mDataValues=mSpell["mDataValues"],
        mSpellCalculations=mSpell["mSpellCalculations"],
        stats=stats,
    )

    calcs = dict()
    for k, v in mSpell["mSpellCalculations"].items():
        try:
            calcs[k] = compute_spell_calc(v, ctx)
        except:
            print("Spell calculation failed", k)
            traceback.print_exc()

    data_vals = dict()
    for v in ctx.mDataValues:
        if "mValues" not in v:
            print("mDataValue has no values", v)
            continue

        key = v["mName"]
        vals = v["mValues"]
        data_vals[key] = vals

    return dict(**data_vals, **calcs)


def compute_spell_calc(
    mSpellCalculation: dict,
    ctx: CalculationContext,
) -> list[float]:
    mult = [1] * (MAX_LEVEL + 1)
    if mMultiplier := mSpellCalculation.get("mMultiplier"):
        mult = compute_subpart(mMultiplier, ctx)

    match mSpellCalculation["__type"]:
        case "GameCalculation":
            assert len(mSpellCalculation["mFormulaParts"]) == 1

            vals = compute_subpart(mSpellCalculation["mFormulaParts"][0], ctx)
        case "GameCalculationModified":
            mMultiplier = mSpellCalculation["mMultiplier"]
            mult_vals = compute_subpart(mMultiplier, ctx)

            spell = mSpellCalculation["mModifiedGameCalculation"]
            vals = compute_spell_calc(ctx.mSpellCalculations[spell], ctx)

            return product_list(mult_vals, vals)

    return product_list(mult, vals)


def compute_subpart(
    mSubpart: dict,
    ctx: CalculationContext,
) -> list[float]:
    match mSubpart["__type"]:
        case "NumberCalculationPart":
            return [
                mSubpart["mNumber"],
            ] * (MAX_LEVEL + 1)
        case "SumOfSubPartsCalculationPart":
            subs = [compute_subpart(x, ctx) for x in mSubpart["mSubparts"]]

            acc = [
                0.0,
            ] * (MAX_LEVEL + 1)
            for xs in subs:
                acc = sum_list(acc, xs)

            return acc
        case "StatBySubPartCalculationPart":
            stat_id = mSubpart["mStat"]
            stat_key = STAT_MAP[stat_id]["key"]
            stat = ctx.stats[stat_key]

            data_vals = compute_subpart(mSubpart["mSubpart"], ctx)
            return [
                get_scaled_stat(stat_id, stat, idx) * x
                for idx, x in enumerate(data_vals)
            ]
        case "SubPartScaledProportionalToStat":
            sub = compute_subpart(mSubpart["mSubpart"], ctx)

            mRatio = mSubpart["mRatio"]
            assert round(mRatio, 3) == 0.01

            # This calculation seems to only be used for AP abilities but the ratio only makes sense
            #   if we were also multiplying by the current AP (all champions start with 100)
            # But we're not so this ratio would make the display value way too small
            # So just gonna skip it here
            return sub
            # return [mRatio * x for x in sub]
        case "NamedDataValueCalculationPart":
            data_key = mSubpart["mDataValue"].lower()
            return ctx.mDataValuesMapped[data_key]["mValues"]
        case "StatByNamedDataValueCalculationPart":
            stat_id = mSubpart["mStat"]
            stat_key = STAT_MAP[stat_id]["key"]
            stat = ctx.stats[stat_key]

            data_key = mSubpart["mDataValue"].lower()
            data_vals = ctx.mDataValuesMapped[data_key]["mValues"]
            return [
                get_scaled_stat(stat_id, stat, idx) * x
                for idx, x in enumerate(data_vals)
            ]
        case "ProductOfSubPartsCalculationPart":
            p1 = compute_subpart(mSubpart["mPart1"], ctx)
            p2 = compute_subpart(mSubpart["mPart2"], ctx)
            return product_list(p1, p2)
        case "{f3cbe7b2}":
            # Recursion case? For set 11, only Irelia has this calculation type
            spell = mSubpart["{88536426}"]
            return compute_spell_calc(ctx.mSpellCalculations[spell], ctx)
        case _:
            print("Unknown subpart type", mSubpart, file=sys.stderr)
            raise Exception()


def get_scaled_stat(stat_id: int, base: float, level: int) -> float:
    scaling = STAT_MAP[stat_id]["scaling"]
    mult = scaling**level
    return base * mult


def product_list(xs: list, ys: list) -> list:
    assert len(xs) == len(ys)
    assert len(xs) == MAX_LEVEL + 1
    return [x * y for x, y in zip(xs, ys)]


def sum_list(xs: list, ys: list) -> list:
    assert len(xs) == len(ys)
    assert len(xs) == MAX_LEVEL + 1
    return [x + y for x, y in zip(xs, ys)]
