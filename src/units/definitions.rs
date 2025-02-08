use once_cell::sync::Lazy;

use super::conversion::*;

impl UnitRegistry {
    fn new_with_definitions() -> Self {
        let mut registry = UnitRegistry::new();
        registry.add_base_dimensions();
        registry.add_base();
        registry.add_volume_units();
        registry.add_derived();
        registry
    }

    fn add_base_dimensions(&mut self) {
        self.add_dimension_simple("length");
        self.add_dimension_simple("mass");
        self.add_dimension_simple("time");
        self.add_dimension_simple("current");
        self.add_dimension_simple("temperature");
        self.add_dimension_simple("amount");
        self.add_dimension_simple("luminosity");
        self.add_dimension_simple("dimensionless");
    }

    fn add_base(&mut self) {
        self.add_unit_simple("meter", "length");
        self.add_unit_simple("kilogram", "mass");
        self.add_unit_simple("second", "time");
        self.add_unit_simple("ampere", "current");
        self.add_unit_simple("kelvin", "temperature");
        self.add_unit_simple("mole", "amount");
        self.add_unit_simple("candela", "luminosity");
        self.add_unit_simple("radian", "dimensionless");
        self.add_unit_simple("bit", "dimensionless");
        self.add_unit_simple("count", "dimensionless");
    }

    fn add_volume_units(&mut self) {
        // Define m3 unit and volume dimension
        self.add_unit(self.try_get_unit("meter").pow(3));
        self.add_dimension("volume", self.try_get_dimension("length").pow(3));

        self.add_unit_deriv("liter", "volume", 1e-3, "meter^3");
        self.add_unit_deriv("cubic_centimeter", "volume", 1e-6, "meter^3");
        self.add_unit_deriv("lambda", "volume", 1e-9, "meter^3");
        self.add_unit_deriv("stere", "volume", 1.0, "meter^3");
    }

    fn add_derived(&mut self) {
        self.add_unit_deriv("turn", "dimensionless", 2.0 * std::f64::consts::PI, "radian");
        self.add_unit_deriv("degree", "dimensionless", 2.0 * std::f64::consts::PI / 360.0, "radian");
        self.add_unit_deriv(
            "arcminute",
            "dimensionless",
            std::f64::consts::PI / (180.0 * 60.0),
            "radian",
        );
        self.add_unit_deriv(
            "arcsecond",
            "dimensionless",
            std::f64::consts::PI / (180.0 * 60.0 * 60.0),
            "radian",
        );
        self.add_unit_deriv(
            "milliarcsecond",
            "dimensionless",
            std::f64::consts::PI / (180.0 * 60.0 * 60.0 * 1000.0),
            "radian",
        );
        self.add_unit_deriv("grade", "dimensionless", std::f64::consts::PI / 200.0, "radian");
        self.add_unit_deriv("mil", "dimensionless", std::f64::consts::PI / 3200.0, "radian");
        self.add_unit_deriv("steradian", "dimensionless", 1.0, "radian");
        self.add_unit_deriv(
            "square_degree",
            "dimensionless",
            (std::f64::consts::PI / 180.0).powi(2),
            "steradian",
        );
        self.add_unit_deriv("baud", "dimensionless", 1.0, "bit");
        self.add_unit_deriv("byte", "dimensionless", 8.0, "bit");
        self.add_unit_deriv("percent", "dimensionless", 0.01, "count");
        self.add_unit_deriv("permille", "dimensionless", 0.001, "count");
        self.add_unit_deriv("ppm", "dimensionless", 1e-6, "count");
        self.add_unit_deriv("angstrom", "length", 1e-10, "meter");
        self.add_unit_deriv("micron", "length", 1e-6, "meter");
        self.add_unit_deriv("fermi", "length", 1e-15, "meter");
        self.add_unit_deriv("light_year", "length", 9.4607e15, "meter");
        self.add_unit_deriv("astronomical_unit", "length", 1.495978707e11, "meter");
        self.add_unit_deriv("parsec", "length", 3.085677581e16, "meter");
        self.add_unit_deriv("nautical_mile", "length", 1852.0, "meter");
        self.add_unit_deriv("bohr", "length", 5.29177210903e-11, "meter");
        self.add_unit_deriv("planck_length", "length", 1.616255e-35, "meter");
    }
}

pub static REGISTRY: Lazy<UnitRegistry> = Lazy::new(|| UnitRegistry::new_with_definitions());

#[cfg(test)]
mod test {
    use super::REGISTRY;

    #[test]
    fn test_new_definitions() {
        assert_eq!(REGISTRY.try_get_unit("meter").simple_unit.name, "meter");
    }
}
