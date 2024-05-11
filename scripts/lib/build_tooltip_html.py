import re
import sys

from html_sanitizer import Sanitizer

"""
Example tooltip:

<titleLeft>Phantom Blade</titleLeft>
<mainText>@ShredTooltipOnly@% <TFTKeyword>Shred</TFTKeyword> the current target for @Duration@&nbsp;seconds and deal
    <magicDamage>@ModifiedDamage@&nbsp;(%i:scaleAP%)</magicDamage> magic damage to them. Heal <scaleHealth>
        @ModifiedHealing@&nbsp;(%i:scaleAP%)</scaleHealth> Health.<br><br>
    <rules>
        <tftbold>Shred:</tftbold> Reduce Magic Resist
    </rules>
</mainText>
<postScriptLeft>Damage: <magicDamage>@ModifiedDamage@ (%i:scaleAP%)</magicDamage><br>Heal: <scaleHealth>
        @ModifiedHealing@ (%i:scaleAP%)</scaleHealth>
</postScriptLeft>
<postScriptRight>[ @Damage1Prefix@@Damage1@%@Damage1Postfix@ / @Damage2Prefix@@Damage2@%@Damage2Postfix@ /
    @Damage3Prefix@@Damage3@%@Damage3Postfix@ ]
    <br>
    [ @FlatHealing1Prefix@@FlatHealing1@%@FlatHealing1Postfix@ /
    @FlatHealing2Prefix@@FlatHealing2@%@FlatHealing2Postfix@ /
    @FlatHealing3Prefix@@FlatHealing3@%@FlatHealing3Postfix@ ]</postScriptRight>
"""


def build_tooltip_html(template: str, variables: dict[str, list[float]]):
    # Variable casing is inconsistent...
    #   eg some placeholders reference AoE but correct name is AOE
    variables = {k.lower(): v for k, v in variables.items()}

    html = template
    html = zip_postscript_left_right(html)
    html = rename_tags(html)
    html = interpolate_variables(html, variables)
    html = f"""
    <div class="tooltip-root">
        <img class="spell-icon" src="$ABILITY_ICON_SRC">
        {html}
    </div>
    """

    allowed_tags = ("h1", "section", "span", "div", "br", "img")
    attributes: dict = {tag: ("class",) for tag in allowed_tags}
    attributes["img"] = ("class", "src")
    sanitizer = Sanitizer(
        dict(
            tags=allowed_tags,
            attributes=attributes,
            empty=set(["br", "img", "span"]),
            separate=set(["div", "span"]),
        )
    )
    return sanitizer.sanitize(html)


def rename_tags(template: str):
    replacements = dict(
        titleLeft=dict(tag="h1", attrs=['class="spell-name"']),
        mainText=dict(tag="section", attrs=['class="spell-description"']),
        TFTKeyword=dict(tag="span", attrs=['class="tft-keyword"']),
        magicDamage=dict(tag="span", attrs=['class="magic-damage"']),
        physicalDamage=dict(tag="span", attrs=['class="physical-damage"']),
        scaleHealth=dict(tag="span", attrs=['class="scale-health"']),
        rules=dict(tag="div", attrs=['class="rules"']),
        tftitemrules=dict(tag="div", attrs=['class="rules"']),
        postScript=dict(tag="div", attrs=['class="post-script"']),
    )

    result = template
    for tag, d in replacements.items():
        open = f"<{tag}>"
        close = f"</{tag}>"
        attrs = " ".join(d["attrs"])

        result = result.replace(open, f"<{d['tag']} {attrs}>")
        result = result.replace(close, f"</{d['tag']}>")

    return result


def interpolate_variables(template: str, variables: dict[str, list[float]]):
    # @some_var*100@ -> (some_var, 100)
    m = re.search(r"@(.*?)(?:\*(\d+(?:\.\d+)?))?@", template)
    if not m:
        return template

    var = m.group(1).lower()
    mult = m.group(2)

    if var in variables:
        # Get vals for some_var at levels 1 to 3
        vals = variables[var][1:4]

        if mult:
            vals = [x * float(mult) for x in vals]

        vals = [round(v) for v in vals]

        if all(v == vals[0] for v in vals):
            val_string = str(vals[0])
        else:
            val_string = " / ".join(str(x) for x in vals)
    elif var.startswith("tftunitproperty"):
        # Some unit properties are mentioned in the stats
        #   but otherwise it's not obvious where this is coming from
        # In any case, it's not super common so just default to whatever
        val_string = "0"
    else:
        # This placeholder is probably one of these
        #   @FlatHealing2Prefix@@FlatHealing2@%@FlatHealing2Postfix@
        val_string = get_generated_variable(variables, var, mult)

    start = template[: m.start()]
    end = interpolate_variables(template[m.end() :], variables)
    return start + val_string + end


def get_generated_variable(variables: dict[str, list[float]], var: str, mult: str):
    m = re.search(r"(\w+)(\d)(prefix|postfix)?", var)
    if not m:
        print("Unknown variable", var, file=sys.stderr)
        raise Exception()

    original_var = m.group(1).lower()
    assert original_var in variables

    level = int(m.group(2))
    modifier = m.group(3)

    if modifier == "prefix":
        assert not mult
        return ""
    elif modifier == "postfix":
        assert not mult
        return ""
    elif modifier is None:
        val = variables[original_var][level]
        if mult:
            val *= float(mult)

        return str(round(val))
    else:
        raise Exception()


def zip_postscript_left_right(template: str) -> str:
    left = re.search(r"<postScriptLeft>(.*)</postScriptLeft>", template)
    right = re.search(r"<postScriptRight>(.*)</postScriptRight>", template)
    if not left or not right:
        raise Exception()

    left_lines = left.group(1).split("<br>")
    right_lines = right.group(1).split("<br>")
    assert len(left_lines) == len(right_lines)

    merged = []
    for l, r in zip(left_lines, right_lines):
        # Don't display total values in post script, only formula / percentages
        #   ie replace  Damage: 625 / 938 / 1406 = 700%  + 100 / 150 / 225%
        #   with        Damage: 700%  + 100 / 150 / 225%
        l2 = re.sub(r": .* = ", ": ", l)
        l = l2

        merged.append(
            f"""
                <span>{l}</span>
                <span>{r}</span>
            """
        )

    joined = "\n".join(merged)
    joined = f"<postScript>{joined}</postScript>"

    assert left.start() < right.end()
    return template[: left.start()] + joined + template[right.end() :]
