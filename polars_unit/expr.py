"""Add unit to Polars expressions API"""

import polars_unit as plu
import polars as pl
from functools import partial

UnitDType = pl.List(pl.Struct({"name": pl.Utf8, "power": pl.Int16}))

def QuantityDtype(dtype: pl.DataType) -> pl.DataType:
    return pl.Struct({
        "value": dtype,
        "unit": UnitDType,
    })

@pl.api.register_expr_namespace("qt")
class QuantityExpr:
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


@pl.api.register_series_namespace("qt")
class QuantitySeries:
    def __init__(self, series: pl.Series) -> None:
        self._series = series

    def with_unit(self, units: list[tuple[str, int]]) -> pl.Expr:
        if not self._series.dtype.is_numeric():
            raise ValueError("Unit supports only numeric types")
        unit_series = pl.Series([[
            {"name": name, "power": power}
            for name, power in units
        ]], dtype=UnitDType)
        return pl.struct(
            value=self._series,
            unit=pl.lit(unit_series),
            eager=True,
        )
