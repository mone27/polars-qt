import pytest
from fractions import Fraction
from polars_qt.units import QtUnit, QtUnits, UnitDType
import pint
import polars as pl


class TestQtUnits:
    @classmethod
    def setup_class(cls):
        cls.ureg = pint.UnitRegistry()

    def test_qtunit_as_lit(self):
        unit = QtUnit(name="meter", power=Fraction(3, 4))
        lit_expr = QtUnits([unit]).as_lit()
        series_from_lit = pl.DataFrame().select(a=lit_expr)["a"]
        expected_series = pl.Series(
            "a",
            [[{"name": "meter", "power": {"numer": 3, "denom": 4}}]],
            dtype=UnitDType,
        )
        assert (series_from_lit == expected_series).all()

    def test_qtunits_initialization(self):
        unit1 = QtUnit(name="meter", power=Fraction(1, 2))
        unit2 = QtUnit(name="second", power=Fraction(3, 4))
        units = QtUnits(units=[unit1, unit2])
        assert units.units == [unit1, unit2]

    def test_pint_to_pqt(self):
        unit = self.ureg("meter ** 0.75 * second**0.5")
        qt_units = QtUnits.from_pint(unit)
        assert len(qt_units.units) == 2
        assert qt_units.units[0].name == "meter"
        assert qt_units.units[0].power == Fraction(3, 4)
        assert qt_units.units[1].name == "second"
        assert qt_units.units[1].power == Fraction(1, 2)

    def test_to_pint(self):
        unit1 = QtUnit(name="meter", power=Fraction(1, 2))
        unit2 = QtUnit(name="second", power=Fraction(3, 4))
        units = QtUnits(units=[unit1, unit2])
        pint_unit = units.to_pint()
        assert str(pint_unit) == "meter ** 0.5 * second ** 0.75"

    def test_convert(self):
        unit1 = QtUnit(name="meter", power=Fraction(1, 1))
        units = QtUnits(units=[unit1])
        new_units, conversion_factor = units.convert("centimeter")
        assert new_units == QtUnits.from_pint(self.ureg("centimeter").units)
        assert conversion_factor == pytest.approx(100.0)  # 1 meter = 100 centimeters


if __name__ == "__main__":
    pytest.main()
