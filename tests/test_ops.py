import polars as pl
import polars_qt as plqt
import pytest


df = pl.DataFrame(
    {
        "qt": pl.Series([1, 2, 3, 4, 5]).qt.with_unit([("m", (1, 1))]),
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
                        {"value": 1, "unittt": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": 2, "unittt": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": 3, "unittt": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                    ]
                )
            }
        )

        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plqt.noop("a"))
        assert "Invalid Quantity" in str(e.value)

    def test_wrong_unit_field_names(self):
        df = pl.DataFrame(
            {
                "a": pl.Series(
                    [
                        {"value": 1, "unit": [{"nameee": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": 2, "unit": [{"nameee": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": 3, "unit": [{"nameee": "m", "power": {"numer": 1, "denom": 1}}]},
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
                        {"value": "1", "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": "2", "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": "3", "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                    ]
                )
            }
        )

        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(unit_noop=plqt.noop("a"))
        assert "Invalid Quantity" in str(e.value)

    def test_multiple_units(self):
        df = pl.DataFrame(
            {
                "qt": pl.Series(
                    [
                        {"value": 1, "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": 2, "unit": [{"name": "m", "power": {"numer": 1, "denom": 1}}]},
                        {"value": 3, "unit": [{"name": "cm", "power": {"numer": 1, "denom": 1}}]}, # different unit
                    ], dtype = plqt.QuantityDtype(pl.Int64)
                )
            }
        )
        with pytest.raises(pl.exceptions.ComputeError) as e:
            df.with_columns(qt_noop=plqt.noop("qt"))
        assert "Expected all units" in str(e.value)


_test_unit = [{"name": "m", "power": {"numer": 1, "denom": 1}}]
class TestUnaryOps:
    def _test_op(self, qt_op, polars_op, exp_unit = _test_unit):
        """Tests that applying unit_op to a column of units is equivalent to applying polars_op to the values"""

        df = pl.DataFrame({'qt': pl.Series([
            {"value": 1, "unit": _test_unit},
            {"value": 2, "unit": _test_unit},
            {"value": 3, "unit": _test_unit},
            {"value": 4, "unit": _test_unit},
            {"value": -5, "unit": _test_unit},
            ], dtype = plqt.QuantityDtype(pl.Int64))})
    
        return (
            df.select(
                qt_op=qt_op("qt"),
                polars_op=polars_op(df["qt"].struct.field("value")),
            )
            .select(
                equal=(
                    (pl.col("qt_op").struct.field("value") == pl.col("polars_op")) & 
                    (pl.col("qt_op").struct.field("unit") == exp_unit)
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

    def test_pow_int(self):
        exp_unit = pl.lit([{"name": "m", "power": {"numer": 2, "denom": 1}}], dtype=plqt.UnitDType)
        assert self._test_op(lambda x: plqt.pow(x, 2), lambda x: x.pow(2), exp_unit)

    def test_sqrt(self):
        exp_unit = pl.lit([{"name": "m", "power": {"numer": 1, "denom": 2}}], dtype=plqt.UnitDType)
        assert self._test_op(plqt.sqrt, lambda x: x.sqrt(), exp_unit)
