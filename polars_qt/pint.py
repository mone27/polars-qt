"""
Converions between polars-qt and pint
"""

from dataclasses import dataclass
from pint import UnitRegistry
import pint
from fractions import Fraction
import polars as pl

ureg = UnitRegistry()


@dataclass
class QtUnit:
    name: str
    power: Fraction

    def as_lit(self) -> pl.Expr:
        return pl.lit(
            {
                "name": self.name,
                "power": {
                    "numer": self.power.numerator,
                    "denom": self.power.denominator,
                },
            }
        )


@dataclass
class QtQuantity:
    value: float
    unit: list[QtUnit]


def pint_to_pqt(unit: pint.Unit) -> list[QtUnit]:
    """
    Convert a pint unit to a list of QtUnit
    """
    return [
        QtUnit(name, Fraction(power).limit_denominator())
        for name, power in unit._units.unit_items()
    ]


def pqt_to_pint(units: list[QtUnit]) -> pint.Unit:
    """
    Convert a list of QtUnit to a pint unit
    """
    return ureg.Unit(
        pint.util.UnitsContainer({unit.name: unit.power for unit in units})
    )
