import polars as pl
import polars_unit as plu
import pytest

unit_dtype = pl.Struct(
    {
        "value": pl.Int64,
        "unit": pl.String,
    }
)


df = pl.DataFrame(
    {
        "unit": pl.Series(
            [
                (1, "m"),
                (2, "m"),
                (3, "m"),
                (4, "m"),
                (5, "m"),
            ],
            dtype=unit_dtype,
        ),
    }
)

class TestNoop:

    def test_noop(self):
        assert (
            df.with_columns(unit_noop=plu.noop("unit"))
            .select(equal=(pl.col("unit_noop") == pl.col("unit")).all())["equal"]
            .item()
        )


    def test_no_struct(self):
        df_no_struct = pl.DataFrame({"a": [1, 2, 3]})
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df_no_struct.with_columns(unit_noop=plu.noop("a"))
        assert "Expected Struct dtype" in str(e.value)


    def test_wrong_struct_n_fields(self):
        df = pl.DataFrame({"a": pl.Series([1, 2, 3], dtype=pl.Struct({"a": pl.Int64}))})
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plu.noop("a"))
        assert "Invalid Unit" in str(e.value)


    def test_wrong_struct_dtypes(self):
        df = pl.DataFrame(
            {
                "a": pl.Series(
                    [(1, 2), (2, 2), (3, 3)],
                    dtype=pl.Struct({"a": pl.Int64, "b": pl.Int64}),
                )
            }
        )
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plu.noop("a"))
        assert "Invalid Unit" in str(e.value)


    def test_wrong_struct_names(self):
        df = pl.DataFrame(
            {
                "a": pl.Series(
                    [(1, "m"), (2, "m"), (3, "m")],
                    dtype=pl.Struct({"a": pl.Int64, "b": pl.String}),
                )
            }
        )
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plu.noop("a"))
        assert "Invalid Unit" in str(e.value)


    def test_wrong_struct_value_dtype(self):
        df = pl.DataFrame(
            {
                "a": pl.Series(
                    [("a", "m"), ("a", "m"), ("a", "m")],
                    dtype=pl.Struct({"unit": pl.String, "value": pl.String}),
                )
            }
        )
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plu.noop("a"))
        assert "Invalid Unit" in str(e.value)

    def test_multiple_units(self):
        df = pl.DataFrame(
            {
                "unit": pl.Series(
                    [
                        (1, "m"),
                        (2, "m"),
                        (3, "cm"),  # inconsistent unit!
                    ],
                    dtype=unit_dtype,
                ),
            }
        )
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plu.noop("unit"))
        assert "Expected all units" in str(e.value)

class TestUnaryOps:

    def test_abs(self):
        df = pl.DataFrame(
            {
                "unit": pl.Series(
                    [
                        (-1, "m"),
                        (-2, "m"),
                        (-3, "m"),
                        (-4, "m"),
                        (-5, "m"),
                    ],
                    dtype=unit_dtype,
                ),
                "abs": pl.Series(
                    [
                        (1, "m"),
                        (2, "m"),
                        (3, "m"),
                        (4, "m"),
                        (5, "m"),
                    ],
                    dtype=unit_dtype,
                ),
            }
        )
        assert (
            df.with_columns(unit_abs=plu.abs("unit"))
            .select(equal=(pl.col("unit_abs") == pl.col("abs")).all())["equal"]
            .item()
        )
