import polars as pl
import polars_unit as plu
import pytest

unit_dtype = pl.Struct(
    {
        "value": pl.Float64,
        "unit": pl.String,
    }
)

df = pl.DataFrame(
    {
        "unit": pl.Series([1, 2, 3, 4, 5]).unit.with_("m"),
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
    def _test_op(self, unit_op, polars_op):
        """Tests that applying unit_op to a column of units is equivalent to applying polars_op to the values"""
        df = pl.DataFrame(
            {
                "unit": pl.Series(
                    [
                        (-1, "m"),
                        (0, "m"),
                        (1, "m"),
                        (2, "m"),
                    ],
                    dtype=unit_dtype,
                ),
            }
        )
        return (
            df.select(
                unit_op=unit_op("unit"),
                polars_op=polars_op(df["unit"].struct.field("value")),
            )
            .select(
                equal=(
                    pl.col("unit_op").struct.field("value") == pl.col("polars_op")
                ).all()
            )["equal"]
            .item()
        )

    def test_abs(self):
        assert self._test_op(plu.abs, lambda x: x.abs())

    def test_sin(self):
        assert self._test_op(plu.sin, lambda x: x.sin())

    def test_arccos(self):
        assert self._test_op(plu.arccos, lambda x: x.arccos())

    def test_mean(self):
        assert self._test_op(plu.mean, lambda x: x.mean())
