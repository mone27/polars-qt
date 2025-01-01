import polars as pl
import pytest


def test_series_creation():
    series = pl.Series([1, 2, 3]).qt.with_unit([("m", (1, 1))])
    to_series = pl.Series(
        [
            {"value": 1, "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
            {"value": 2, "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
            {"value": 3, "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
        ]
    )
    assert (series == to_series).all()


def test_series_wrong_num_type():
    with pytest.raises(ValueError) as e:
        pl.Series(["a", "b", "c"]).qt.with_unit([("m", (1, 1))])
    assert "Unit supports" in str(e.value)
