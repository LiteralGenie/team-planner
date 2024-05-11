import sys

STAT_MAP = {
    1: dict(key="baseArmor", scaling=1),
    2: dict(key="baseDamage", scaling=1.5),
    3: dict(key="attackSpeed", scaling=1),
    5: dict(key="baseSpellBlock", scaling=1),
    11: dict(key="baseHP", scaling=1.8),
}

MAX_LEVEL = 6


def compute_spell_variables(
    mSpell: dict,
    stats: dict,
) -> dict[str, list[float]]:
    mDataValuesMapped = {x["mName"].lower(): x for x in mSpell["mDataValues"]}

    calcs = {
        k: compute_spell_calc(
            v,
            mSpell["mSpellCalculations"],
            mDataValuesMapped,
            stats,
        )
        for k, v in mSpell["mSpellCalculations"].items()
    }

    return dict(**mDataValuesMapped, **calcs)


def compute_spell_calc(
    mSpellCalculation: dict,
    mSpellCalculationsAll: dict,
    mDataValuesMapped: dict,
    stats: dict,
) -> list[float]:
    mult = [1] * (MAX_LEVEL + 1)
    if mMultiplier := mSpellCalculation.get("mMultiplier"):
        mult = compute_subpart(mMultiplier, mDataValuesMapped, stats)

    match mSpellCalculation["__type"]:
        case "GameCalculation":
            assert len(mSpellCalculation["mFormulaParts"]) == 1

            vals = compute_subpart(
                mSpellCalculation["mFormulaParts"][0], mDataValuesMapped, stats
            )
        case "GameCalculationModified":
            mMultiplier = mSpellCalculation["mMultiplier"]
            mult_vals = compute_subpart(mMultiplier, mDataValuesMapped, stats)

            vals = compute_subpart(
                mSpellCalculationsAll["mModifiedGameCalculation"],
                mDataValuesMapped,
                stats,
            )

            return product_list(mult_vals, vals)

    return product_list(mult, vals)


def compute_subpart(
    mSubpart: dict,
    mDataValuesMapped: dict,
    stats: dict,
) -> list[float]:
    match mSubpart["__type"]:
        case "NumberCalculationPart":
            return [
                mSubpart["mNumber"],
            ] * (MAX_LEVEL + 1)
        case "SumOfSubPartsCalculationPart":
            subs = [
                compute_subpart(x, mDataValuesMapped, stats)
                for x in mSubpart["mSubparts"]
            ]

            acc = [
                0.0,
            ] * (MAX_LEVEL + 1)
            for xs in subs:
                acc = sum_list(acc, xs)

            return acc
        case "StatBySubPartCalculationPart":
            stat_id = mSubpart["mStat"]
            stat_key = STAT_MAP[stat_id]["key"]
            stat = stats[stat_key]

            data_vals = compute_subpart(mSubpart["mSubpart"], mDataValuesMapped, stats)
            return [
                get_scaled_stat(stat_id, stat, idx) * x
                for idx, x in enumerate(data_vals)
            ]
        case "SubPartScaledProportionalToStat":
            sub = compute_subpart(mSubpart["mSubpart"], mDataValuesMapped, stats)
            mRatio = mSubpart["mRatio"]
            return [mRatio * x for x in sub]
        case "NamedDataValueCalculationPart":
            data_key = mSubpart["mDataValue"].lower()
            return mDataValuesMapped[data_key]["mValues"]
        case "StatByNamedDataValueCalculationPart":
            stat_id = mSubpart["mStat"]
            stat_key = STAT_MAP[stat_id]["key"]
            stat = stats[stat_key]

            data_key = mSubpart["mDataValue"].lower()
            data_vals = mDataValuesMapped[data_key]["mValues"]
            return [
                get_scaled_stat(stat_id, stat, idx) * x
                for idx, x in enumerate(data_vals)
            ]
        # case 'ProductOfSubPartsCalculationPart':
        #     p1 = compute_subpart(mSubpart['mPart1'], mDataValuesMapped, stats)
        #     p2 = compute_subpart(mSubpart['mPart2'], mDataValuesMapped, stats)
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
