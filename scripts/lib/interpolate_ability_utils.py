from dataclasses import dataclass
from typing import Literal, TypeAlias, TypedDict

BASE_AP = 100


class Variable(TypedDict):
    name: str
    # Variables can technically have null values,
    # but those shouldn't be necessary to calculate any placeholders
    # (probably an oversight that they're included)
    value: list[float]


VariableMap: TypeAlias = dict[str, Variable]


ScalingStat: TypeAlias = Literal["ap", "ad", "hp", "speed", "armor", "resist"]


@dataclass
class Term:
    variable_name: str | None
    stat: ScalingStat | None = None
    div: float = 1

    def __post_init__(self):
        assert self.variable_name != None or self.stat != None

    def compute(self, vars: dict[str, float], stats: dict) -> float:
        val = 1

        if self.stat:
            match self.stat:
                case "ad":
                    val = stats["damage"]
                case "ap":
                    val = BASE_AP
                case "speed":
                    val = stats["attackSpeed"]
                case "resist":
                    val = stats["magicResist"]
                case _:
                    val = stats[self.stat]

        if self.variable_name:
            val *= vars[self.variable_name]

        return val / self.div


def scale_stat(val: float, level: int, mult: float):
    return val * (mult ** (level - 1))
