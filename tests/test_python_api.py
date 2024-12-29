import polars_unit as plu
import polars as pl
import pytest

def test_series_creation():
    series = pl.Series([1,2,3]).unit.with_("m")
    to_series = pl.Series([
        (1, "m"),
        (2, "m"),
        (3, "m")
    ], dtype=pl.Struct({"value" : pl.Int64, "unit": pl.String})
    )
    assert (series == to_series).all()

def test_series_wrong_num_type():
    with pytest.raises(ValueError) as e:
        pl.Series(["a", "b", "c"]).unit.with_("m")
    assert "Unit supports" in str(e.value)