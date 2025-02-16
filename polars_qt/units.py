"""
Base classes to handle units
"""

import polars as pl
from dataclasses import dataclass
import pint
from fractions import Fraction


__all__ = ["QuantityDtype", "UnitDType"]

UnitDType = pl.List(
    pl.Struct(
        {"name": pl.Utf8, "power": pl.Struct({"numer": pl.Int64, "denom": pl.Int64})}
    )
)


def QuantityDtype(dtype: pl.DataType) -> pl.DataType:
    return pl.Struct(
        {
            "value": dtype,
            "unit": UnitDType,
        }
    )


@dataclass
class QtUnit:
    name: str
    power: Fraction

    def as_dict(self) -> dict:
        return {
            "name": self.name,
            "power": {
                "numer": self.power.numerator,
                "denom": self.power.denominator,
            },
        }


@dataclass
class QtUnits:
    units: list[QtUnit]

    def as_lit(self) -> pl.Expr:
        return pl.lit([unit.as_dict() for unit in self.units], dtype=UnitDType)

    @classmethod
    def from_quantity_scalar(cls, scalar: dict) -> "QtUnits":
        """from a polars Quantity scaler (i.e. a single row), ignores the value"""
        return cls(
            [
                QtUnit(
                    unit["name"],
                    Fraction(unit["power"]["numer"], unit["power"]["denom"]),
                )
                for unit in scalar["unit"]
            ]
        )

    @classmethod
    def from_pint(cls, unit: pint.Unit) -> "QtUnits":
        return cls(
            [
                QtUnit(name, Fraction(power).limit_denominator())
                for name, power in unit._units.unit_items()
            ]
        )

    def to_pint(self) -> pint.Unit:
        return pint.Unit(
            pint.util.UnitsContainer({unit.name: unit.power for unit in self.units})
        )

    def convert(self, to_unit: pint.Unit | str) -> ("QtUnits", float):
        """
        Convert the units to another unit.
        Returns the new units and the conversion factor
        """
        pint_quantity = 1 * self.to_pint()
        converted = pint_quantity.to(to_unit)
        return QtUnits.from_pint(converted.units), converted.magnitude
