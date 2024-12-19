import polars as pl
from polars_unit import noop
import pytest

unit_dtype = pl.Struct({
    'value': pl.Int64,
    'unit': pl.String,
})


df = pl.DataFrame({
    'unit': pl.Series([
        (1, 'm'),
        (2, 'm'),
        (3, 'm'),
        (4, 'm'),
        (5, 'm'),
    ], dtype=unit_dtype),
})



def test_noop():
    assert(df.with_columns(
        unit_noop = noop("unit")
    ).select(
        equal = (pl.col("unit_noop") == pl.col("unit")).all()
    )['equal'].item())
    

def test_no_struct():
    df_no_struct = pl.DataFrame({'a': [1, 2, 3]})
    with pytest.raises(pl.exceptions.ComputeError) as e:
        df_no_struct.with_columns(
            unit_noop = noop("a")
        )
    assert "expected Struct dtype" in str(e.value)
