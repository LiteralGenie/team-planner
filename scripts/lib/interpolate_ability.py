import re
from dataclasses import dataclass

from .constants import FORMULAS, NUM_STAR_LEVELS, STAT_LEVEL_MULTS
from .interpolate_ability_utils import Variable, scale_stat


def interpolate_all(
    champion_id: str,
    champion_stats: dict,
    variables: list[Variable],
    text: str,
) -> str:
    result = text
    while True:
        m = re.search(r"@(\w+(?:\*\d+)?)@", result)
        if not m:
            break

        val = interpolate_expression(
            champion_id,
            champion_stats,
            variables,
            m.group(1),
        )

        if all(v == val[0] for v in val[1:]):
            val_str = str(int(val[0]))
        else:
            val_str = " / ".join(str(int(x)) for x in val)

        result = result[: m.start()] + val_str + result[m.end() :]

    return result


def interpolate_expression(
    champion_id: str,
    champion_stats: dict,
    variables: list[Variable],
    expr: str,
) -> list[float]:
    split = expr.split("*")

    formula = split[0]
    formula_val = interpolate_single(
        champion_id,
        champion_stats,
        variables,
        formula,
    )

    if len(split) > 1:
        formula_val = [x * int(split[1]) for x in formula_val]

    return formula_val


def interpolate_single(
    champion_id: str,
    champion_stats: dict,
    variables: list[Variable],
    target_variable: str,
) -> list[float]:
    vars_by_id = {x["name"]: x for x in variables if x["value"] != None}

    override = FORMULAS.get(champion_id.lower(), dict()).get(target_variable)

    if not override:
        # If no override, find a variable with the same name as placeholder and return its values
        var = vars_by_id[target_variable]
        return [var["value"][idx] for idx in range(1, NUM_STAR_LEVELS + 1)]
    else:
        values = []

        for idx in range(1, NUM_STAR_LEVELS + 1):
            scaled_stats = {
                k: scale_stat(v, idx, STAT_LEVEL_MULTS.get(k, 1))
                for k, v in champion_stats.items()
            }
            scaled_vars = {k: v["value"][idx] for k, v in vars_by_id.items()}

            term_values = [term.compute(scaled_vars, scaled_stats) for term in override]
            values.append(sum(term_values))

        return values
