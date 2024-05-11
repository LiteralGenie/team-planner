import re
import sys
from typing import TypeAlias

from html_sanitizer import Sanitizer

ConditionalTraitData: TypeAlias = dict
EffectsByName: TypeAlias = dict[str, dict]


class InterpolationContext:
    trait: dict
    effect_tallies: dict[str, dict]

    def __init__(self, trait: dict):
        self.trait = trait
        self.effect_tallies = self._init_effect_tallies(trait)

    def _init_effect_tallies(self, trait: dict):
        result = dict()

        for set in trait["innate_trait_sets"]:
            for effect in set["effect_amounts"]:
                name = effect["name"]
                assert effect["format_string"] == ""

                result.setdefault(name, dict(next_idx=0, vals=[]))
                result[name]["vals"].append(effect["value"])

        result["min_units"] = dict(next_idx=0, vals=[])
        result["max_units"] = dict(next_idx=0, vals=[])
        for set in trait["conditional_trait_sets"]:
            result["min_units"]["vals"].append(set["min_units"])

            if "max_units" in set:
                result["max_units"]["vals"].append(set["max_units"])

            for effect in set["effect_amounts"]:
                name = effect["name"]
                assert effect["format_string"] == ""

                result.setdefault(name, dict(next_idx=0, vals=[]))
                result[name]["vals"].append(effect["value"])

        return result

    def consume(self, effect_name: str) -> float:
        tally = self.effect_tallies[effect_name]

        if tally["next_idx"] == len(tally["vals"]):
            raise Exception()

        idx = tally["next_idx"]
        tally["next_idx"] += 1
        return tally["vals"][idx]


def build_trait_html(trait: dict):
    ctx = InterpolationContext(trait)

    html = trait["tooltip_text"]

    html = expand_row(html, trait)
    html = interpolate(html, ctx)

    html = html.replace("<row>", '<span clsas="conditional-effect">')
    html = html.replace("</row>", "</span>")

    html = f'<div class="trait-tooltip">{html}</div>'
    html = sanitize(html)

    return html


def sanitize(html: str) -> str:
    allowed_tags = ("div", "span", "br")
    attributes: dict = {tag: ("class",) for tag in allowed_tags}
    sanitizer = Sanitizer(
        dict(
            tags=allowed_tags,
            attributes=attributes,
            empty=set(["br", "span"]),
            separate=set(["div", "span"]),
        )
    )
    return sanitizer.sanitize(html)


def interpolate(template: str, ctx: InterpolationContext):
    # @some_var*100@ -> (some_var, 100)
    m = re.search(r"@(.*?)(?:\*(\d+(?:\.\d+)?))?@", template)
    if not m:
        return template

    var = m.group(1)
    mult = m.group(2)

    match var:
        case "MinUnits":
            val = ctx.consume("min_units")
        case "MaxUnits":
            val = ctx.consume("max_units")
        case _:
            if var.startswith("TFTUnitProperty"):
                val = 0
            else:
                val = ctx.consume(var)

    if mult:
        val *= float(mult)

    val = round(val)

    start = template[: m.start()]
    end = interpolate(template[m.end() :], ctx)
    return start + str(val) + end


def expand_row(text: str, trait: dict):
    m = re.search(r"<expandRow>(.*)</expandRow>", text)
    if not m:
        return text

    template = m.group(1)

    rows = [template for _ in trait["conditional_trait_sets"]]

    rows = [f"<br><row>{row}</row>" for row in rows]

    start = text[: m.start()]
    joined = "\n".join(rows)
    end = text[m.end() :]
    return start + joined + end
