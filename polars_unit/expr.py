"""Add unit to Polars expressions API"""

import polars_unit as plu
import polars as pl
from functools import partial


@pl.api.register_expr_namespace("unit")
class UnitExpr:
    def __init__(self, expr: pl.Expr) -> None:
        self._expr = expr

    def __getattr__(self, name: str, *args, **kwargs) -> pl.Expr:
        return partial(getattr(plu.functions, name), self._expr, *args, **kwargs)

    def __add__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.add(self._expr, other)

    def __radd__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.add(other, self._expr)

    def __mul__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.mul(self._expr, other)

    def __rmul__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.mul(other, self._expr)

    def __sub__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.sub(self._expr, other)

    def __rsub__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.sub(other, self._expr)

    def __truediv__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.div(self._expr, other)

    def __rtruediv__(self, other: pl.Expr) -> pl.Expr:
        return plu.functions.div(other, self._expr)

    def __abs__(self) -> pl.Expr:
        return plu.functions.abs(self._expr)


@pl.api.register_series_namespace("unit")
class UnitSeries:
    def __init__(self, series: pl.Series) -> None:
        self._series = series

    def with_(self, unit: str) -> pl.Expr:
        if not self._series.dtype.is_numeric():
            raise ValueError("Unit supports only numeric types")
        return pl.struct(
            value=self._series,
            unit=pl.lit(unit).cast(pl.String),
            eager=True,
        )
