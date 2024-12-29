import polars as pl
import polars_unit as plqt
import pytest


df = pl.DataFrame(
    {
        "qt": pl.Series([1, 2, 3, 4, 5]).qt.with_unit([("m", 1)]),
    }
)


class TestNoop:
    def test_noop(self):
        assert (
            df.with_columns(qt_noop=plqt.noop("qt"))
            .select(equal=(pl.col("qt_noop") == pl.col("qt")).all())["equal"]
            .item()
        )

    def test_no_struct(self):
        df_no_struct = pl.DataFrame({"a": [1, 2, 3]})
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df_no_struct.with_columns(qt_noop=plqt.noop("a"))
        assert "Expected Struct dtype" in str(e.value)

    def test_wrong_struct_n_fields(self):
        df = pl.DataFrame({"a": pl.Series([1, 2, 3], dtype=pl.Struct({"a": pl.Int64}))})
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plqt.noop("a"))
        assert "Invalid Quantity" in str(e.value)

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
            df.with_columns(unit_noop=plqt.noop("a"))
        assert "Invalid Unit" in str(e.value)

    def test_wrong_struct_names(self):
        df = pl.DataFrame(
            {
                "a": pl.Series(
                    [
                        {"value": 1, "unittt": [{"name": "m", "power": 1}]},
                        {"value": 2, "unittt": [{"name": "m", "power": 1}]},
                        {"value": 3, "unittt": [{"name": "m", "power": 1}]},
                    ]
                )
            }
        )

        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plqt.noop("a"))
        assert "Invalid Unit" in str(e.value)

    def test_wrong_unit_field_names(self):
        df = pl.DataFrame(
            {
                "a": pl.Series(
                    [
                        {"value": 1, "unit": [{"nameee": "m", "power": 1}]},
                        {"value": 2, "unit": [{"nameee": "m", "power": 1}]},
                        {"value": 3, "unit": [{"nameee": "m", "power": 1}]},
                    ]
                )
            }
        )

        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plqt.noop("a"))
        assert "Invalid Unit" in str(e.value)

    def test_wrong_struct_value_dtype(self):
        df = pl.DataFrame(
            {
                "a": pl.Series(
                    [
                        {"value": "1", "unit": [{"nameee": "m", "power": 1}]},
                        {"value": "2", "unit": [{"nameee": "m", "power": 1}]},
                        {"value": "3", "unit": [{"nameee": "m", "power": 1}]},
                    ]
                )
            }
        )

        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plqt.noop("a"))
        assert "Invalid Unit" in str(e.value)

    def test_multiple_units(self):
        df = pl.DataFrame(
            {
                "qt": pl.Series(
                    [
                        {"value": 1, "unit": [{"name": "m", "power": 1}]},
                        {"value": 2, "unit": [{"name": "m", "power": 1}]},
                        {"value": 3, "unit": [{"name": "cm", "power": 1}]}, # different unit
                    ], dtype = plqt.QuantityDtype(pl.Int64)
                )
            }
        )
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(qt_noop=plqt.noop("qt"))
        assert "Expected all units" in str(e.value)


class TestUnaryOps:
    def _test_op(self, qt_op, polars_op):
        """Tests that applying unit_op to a column of units is equivalent to applying polars_op to the values"""

        df = pl.DataFrame({'qt': pl.Series([
            {"value": 1, "unit": [{"name": "m", "power":1}]},
            {"value": 2, "unit": [{"name": "m", "power":1}]},
            {"value": 3, "unit": [{"name": "m", "power":1}]},
            {"value": 4, "unit": [{"name": "m", "power":1}]},
            {"value": -5, "unit": [{"name": "m", "power":1}]}
            ], dtype = plqt.QuantityDtype(pl.Int64))})
        return (
            df.select(
                qt_op=qt_op("qt"),
                polars_op=polars_op(df["qt"].struct.field("value")),
            )
            .select(
                equal=(
                    pl.col("qt_op").struct.field("value") == pl.col("polars_op")
                ).all()
            )["equal"]
            .item()
        )

    def test_abs(self):
        assert self._test_op(plqt.abs, lambda x: x.abs())

    def test_sin(self):
        assert self._test_op(plqt.sin, lambda x: x.sin())

    def test_arccos(self):
        assert self._test_op(plqt.arccos, lambda x: x.arccos())

    def test_mean(self):
        assert self._test_op(plqt.mean, lambda x: x.mean())
