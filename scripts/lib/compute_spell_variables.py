import sys

STAT_MAP = {
    1: dict(key="baseArmor", scaling=1),
    2: dict(key="baseDamage", scaling=1.5),
    5: dict(key="baseSpellBlock", scaling=1),
    11: dict(key="baseHP", scaling=1.8),
}


def compute_spell_variables(
    mSpell: dict,
    stats: dict,
) -> dict[str, list[float]]:
    mDataValuesMapped = {x["mName"]: x for x in mSpell["mDataValues"]}

    calcs = {
        k: compute_spell_calc(v, mDataValuesMapped, stats)
        for k, v in mSpell["mSpellCalculations"].items()
    }

    return dict(**mDataValuesMapped, **calcs)


def compute_spell_calc(
    mSpellCalculation: dict, mDataValuesMapped: dict, stats: dict
) -> list[float]:
    mult = 1
    if mMultiplier := mSpellCalculation.get("mMultiplier"):
        mult = compute_multiplier(mMultiplier)

    assert len(mSpellCalculation["mFormulaParts"]) == 1
    vals = compute_subpart(
        mSpellCalculation["mFormulaParts"][0], mDataValuesMapped, stats
    )

    return [mult * x for x in vals]


def compute_multiplier(mMultiplier: dict) -> float:
    match mMultiplier["__type"]:
        case "NumberCalculationPart":
            return mMultiplier["mNumber"]
        case _:
            print("Unknown multiplier type", mMultiplier, file=sys.stderr)
            raise Exception()


def compute_subpart(
    mSubpart: dict,
    mDataValuesMapped: dict,
    stats: dict,
) -> list[float]:
    match mSubpart["__type"]:
        case "SumOfSubPartsCalculationPart":
            subs = [
                compute_subpart(x, mDataValuesMapped, stats)
                for x in mSubpart["mSubparts"]
            ]

            return [sum(xs) for xs in subs]
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
            data_key = mSubpart["mDataValue"]
            return mDataValuesMapped[data_key]["mValues"]
        case "StatByNamedDataValueCalculationPart":
            stat_id = mSubpart["mStat"]
            stat_key = STAT_MAP[stat_id]["key"]
            stat = stats[stat_key]

            data_vals = mDataValuesMapped[mSubpart["mDataValue"]]
            return [
                get_scaled_stat(stat_id, stat, idx) * x
                for idx, x in enumerate(data_vals["mValues"])
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
