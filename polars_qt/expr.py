"""Add unit to Polars expressions API"""

from typing import Optional
import polars_qt as pqt
import polars as pl
from functools import partial
from .units import UnitDType


@pl.api.register_expr_namespace("qt")
class QuantityExpr:
    def __init__(self, expr: pl.Expr) -> None:
        self._expr = expr

    def __getattr__(self, name: str, *args, **kwargs) -> pl.Expr:
        return partial(getattr(pqt.functions, name), self._expr, *args, **kwargs)

    def __add__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.add(self._expr, other)

    def __radd__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.add(other, self._expr)

    def __mul__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.mul(self._expr, other)

    def __rmul__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.mul(other, self._expr)

    def __sub__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.sub(self._expr, other)

    def __rsub__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.sub(other, self._expr)

    def __truediv__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.div(self._expr, other)

    def __rtruediv__(self, other: pl.Expr) -> pl.Expr:
        return pqt.functions.div(other, self._expr)

    def __abs__(self) -> pl.Expr:
        return pqt.functions.abs(self._expr)


@pl.api.register_series_namespace("qt")
class QuantitySeries:
    def __init__(self, series: pl.Series) -> None:
        self._series = series

    def with_unit(self, units: list[tuple[str, Optional[tuple[int, int]]]]) -> pl.Expr:
        if not self._series.dtype.is_numeric():
            raise ValueError("Unit supports only numeric types")
        # default to (1, 1) if no power is provided
        # units = [(unit[0], (1, 1)  else unit) for unit in units]
        # default denominator to 1 if not provided
        # units = [(name, (power[0], 1) if len(power)==1 else power) for name, power in units]
        unit_series = pl.Series(
            [
                [
                    {"name": name, "power": {"numer": power[0], "denom": power[1]}}
                    for name, power in units
                ]
            ],
            dtype=UnitDType,
        )
        return pl.struct(
            value=self._series,
            unit=pl.lit(unit_series),
            eager=True,
        )
