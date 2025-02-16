import polars as pl
import numpy as np


def test_conversion_simple():
    df = pl.DataFrame(
        {
            "meter": pl.Series([1.0, 2.0, 3.0]).qt.with_unit([("meter", (1, 1))]),
            "expected_foot": pl.Series(
                [3.28084, 2 * 3.28084, 3 * 3.28084]
            ).qt.with_unit([("foot", (1, 1))]),
        }
    )
    df = df.with_columns(actual_foot=pl.col("meter").qt.convert("foot"))
    print(df)
    assert (
        df["actual_foot"].struct.field("unit")
        == df["expected_foot"].struct.field("unit")
    ).all()
    assert np.allclose(
        df["actual_foot"].struct.field("value").to_numpy(),
        df["expected_foot"].struct.field("value").to_numpy(),
    )
