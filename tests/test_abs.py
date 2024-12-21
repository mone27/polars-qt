import polars as pl
from polars_unit import abs
import pytest

unit_dtype = pl.Struct({
    'value': pl.Int64,
    'unit': pl.String,
})


df = pl.DataFrame({
    'unit': pl.Series([
        (-1, 'm'),
        (-2, 'm'),
        (-3, 'm'),
        (-4, 'm'),
        (-5, 'm'),
    ], dtype=unit_dtype),
    'abs': pl.Series([
        (1, 'm'),
        (2, 'm'),
        (3, 'm'),
        (4, 'm'),
        (5, 'm'),
    ], dtype=unit_dtype),
})

def test_abs():
    assert(df.with_columns(
        unit_abs = abs("unit")
    ).select(
        equal = (pl.col("unit_abs") == pl.col("abs")).all()
    )['equal'].item())