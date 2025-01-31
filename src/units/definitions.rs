use std::collections::{HashMap, HashSet};

use num_rational::Rational64;
use once_cell::sync::Lazy;

use super::conversion::*;

// macro_rules! dimension {
//     ($name:ident) => {
//         dimensions.insert(Dimension {
//                 dimensions: vec![($name.to_string(), Rational64::new(1, 1))],
//         });
//     };
//     // ($name:ident, $numer:$)
// }

// pub static DIMENSIONS: Lazy<HashSet<Dimension>> = Lazy::new(|| {
//     let mut dimensions = HashSet::new();
//     dimension!(length);
//     dimension!(mass);
//     dimension!(time);
//     dimension!(current);
//     dimension!(temperature);
//     dimension!(amount);
//     dimension!(luminosity);
//     dimensions
// });

macro_rules! simple_unit {
    ($units:ident, $name:expr, $dimension:expr) => {
        $units.insert(
            $name.to_string(),
            Unit {
                unit: (BaseUnit {
                    name: $name.to_string(),
                    dimension: $dimension.clone(),
                }),
                conversion: Box::new(None),
            },
        );
    };
}

macro_rules! unit {
    ($name:expr, $dimension:expr, $conv:expr, $units:ident, $dimensions:ident) => {
        $units.insert(
            "$name".to_string(),
            Unit {
                unit: BaseUnit {
                    name: $name.to_string(),
                    dimension: $dimensions[$dimension].clone(),
                },
                conversion: Box::new(Some($conv)),
            },
        );
    };
}

impl UnitRegistry {
    pub fn new() -> Self {
        let mut dimensions = HashMap::new();
        UnitRegistry::add_base_dimensions(&mut dimensions);
        let mut units = HashMap::new();
        UnitRegistry::add_base(&mut units, &dimensions);
        Self { units, dimensions }
    }

    fn add_base_dimensions(dimensions: &mut HashMap<String, Dimension>) {
        dimensions.insert(
            "length".to_string(),
            Dimension {
                dimensions: vec![("length".to_string(), Rational64::new(1, 1))],
            },
        );
        dimensions.insert(
            "mass".to_string(),
            Dimension {
                dimensions: vec![("mass".to_string(), Rational64::new(1, 1))],
            },
        );
        dimensions.insert(
            "time".to_string(),
            Dimension {
                dimensions: vec![("time".to_string(), Rational64::new(1, 1))],
            },
        );
        dimensions.insert(
            "current".to_string(),
            Dimension {
                dimensions: vec![("current".to_string(), Rational64::new(1, 1))],
            },
        );

        dimensions.insert(
            "temperature".to_string(),
            Dimension {
                dimensions: vec![("temperature".to_string(), Rational64::new(1, 1))],
            },
        );
        dimensions.insert(
            "amount".to_string(),
            Dimension {
                dimensions: vec![("amount".to_string(), Rational64::new(1, 1))],
            },
        );
        dimensions.insert(
            "luminosity".to_string(),
            Dimension {
                dimensions: vec![("luminosity".to_string(), Rational64::new(1, 1))],
            },
        );
        dimensions.insert(
            "dimensionless".to_string(),
            Dimension {
                dimensions: Vec::new(),
            },
        );
    }

    fn add_base(units: &mut HashMap<String, Unit>, dimensions: &HashMap<String, Dimension>) {
        simple_unit!(units, "meter", dimensions["length"]);
        simple_unit!(units, "kilogram", dimensions["mass"]);
        simple_unit!(units, "second", dimensions["time"]);
        simple_unit!(units, "ampere", dimensions["current"]);
        simple_unit!(units, "kelvin", dimensions["temperature"]);
        simple_unit!(units, "mole", dimensions["amount"]);
        simple_unit!(units, "candela", dimensions["luminosity"]);
        simple_unit!(units, "radian", dimensions["dimensionless"]);
        simple_unit!(units, "bit", dimensions["dimensionless"]);
        simple_unit!(units, "count", dimensions["dimensionless"]);
    }

    fn add_derived(units: &mut HashMap<String, Unit>, dimensions: &HashMap<String, Dimension>) {
        unit!(
            "turn",
            "dimensionless",
            Conversion::new(2.0 * std::f64::consts::PI, units["radian"].clone()),
            units,
            dimensions
        );

        unit!(
            "degree",
            "dimensionless",
            Conversion::new(2.0 * std::f64::consts::PI / 360.0, units["radian"].clone()),
            units,
            dimensions
        );

        unit!(
            "arcminute",
            "dimensionless",
            Conversion::new(
                std::f64::consts::PI / (180.0 * 60.0),
                units["radian"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "arcsecond",
            "dimensionless",
            Conversion::new(
                std::f64::consts::PI / (180.0 * 60.0 * 60.0),
                units["radian"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "milliarcsecond",
            "dimensionless",
            Conversion::new(
                std::f64::consts::PI / (180.0 * 60.0 * 60.0 * 1000.0),
                units["radian"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "grade",
            "dimensionless",
            Conversion::new(std::f64::consts::PI / 200.0, units["radian"].clone()),
            units,
            dimensions
        );

        unit!(
            "mil",
            "dimensionless",
            Conversion::new(std::f64::consts::PI / 3200.0, units["radian"].clone()),
            units,
            dimensions
        );

        unit!(
            "steradian",
            "dimensionless",
            Conversion::new(1.0, units["radian"].clone()),
            units,
            dimensions
        );

        unit!(
            "square_degree",
            "dimensionless",
            Conversion::new(
                (std::f64::consts::PI / 180.0).powi(2),
                units["steradian"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "baud",
            "dimensionless",
            Conversion::new(1.0, units["bit"].clone()),
            units,
            dimensions
        );

        unit!(
            "byte",
            "dimensionless",
            Conversion::new(8.0, units["bit"].clone()),
            units,
            dimensions
        );

        unit!(
            "percent",
            "dimensionless",
            Conversion::new(0.01, units["dimensionless"].clone()),
            units,
            dimensions
        );

        unit!(
            "permille",
            "dimensionless",
            Conversion::new(0.001, units["dimensionless"].clone()),
            units,
            dimensions
        );

        unit!(
            "ppm",
            "dimensionless",
            Conversion::new(1e-6, units["dimensionless"].clone()),
            units,
            dimensions
        );

        unit!(
            "angstrom",
            "length",
            Conversion::new(1e-10, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "micron",
            "length",
            Conversion::new(1e-6, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "fermi",
            "length",
            Conversion::new(1e-15, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "light_year",
            "length",
            Conversion::new(9.4607e15, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "astronomical_unit",
            "length",
            Conversion::new(1.495978707e11, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "parsec",
            "length",
            Conversion::new(3.085677581e16, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "nautical_mile",
            "length",
            Conversion::new(1852.0, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "bohr",
            "length",
            Conversion::new(5.29177210903e-11, units["meter"].clone()),
            units,
            dimensions
        );

        unit!(
            "planck_length",
            "length",
            Conversion::new(1.616255e-35, units["meter"].clone()),
            units,
            dimensions
        );
        unit!(
            "metric_ton",
            "mass",
            Conversion::new(1e3, units["kilogram"].clone()),
            units,
            dimensions
        );
        unit!(
            "unified_atomic_mass_unit",
            "mass",
            Conversion::new(1.66053906660e-27, units["kilogram"].clone()),
            units,
            dimensions
        );
        unit!(
            "dalton",
            "mass",
            Conversion::new(1.66053906660e-27, units["kilogram"].clone()),
            units,
            dimensions
        );
        unit!(
            "grain",
            "mass",
            Conversion::new(64.79891e-6, units["kilogram"].clone()),
            units,
            dimensions
        );
        unit!(
            "gamma_mass",
            "mass",
            Conversion::new(1e-9, units["kilogram"].clone()),
            units,
            dimensions
        );
        unit!(
            "carat",
            "mass",
            Conversion::new(200e-6, units["kilogram"].clone()),
            units,
            dimensions
        );
        unit!(
            "planck_mass",
            "mass",
            Conversion::new(
                (1.054571817e-34 * 299792458.0 / 6.67430e-11).sqrt(),
                units["kilogram"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "minute",
            "time",
            Conversion::new(60.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "hour",
            "time",
            Conversion::new(3600.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "day",
            "time",
            Conversion::new(86400.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "week",
            "time",
            Conversion::new(604800.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "fortnight",
            "time",
            Conversion::new(1209600.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "year",
            "time",
            Conversion::new(31557600.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "month",
            "time",
            Conversion::new(2629800.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "century",
            "time",
            Conversion::new(3155760000.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "millennium",
            "time",
            Conversion::new(31557600000.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "eon",
            "time",
            Conversion::new(3.15576e16, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "shake",
            "time",
            Conversion::new(1e-8, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "svedberg",
            "time",
            Conversion::new(1e-13, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "atomic_unit_of_time",
            "time",
            Conversion::new(2.4188843265857e-17, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "gregorian_year",
            "time",
            Conversion::new(31556952.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "sidereal_year",
            "time",
            Conversion::new(31558149.7635456, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "tropical_year",
            "time",
            Conversion::new(31556925.216, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "common_year",
            "time",
            Conversion::new(31536000.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "leap_year",
            "time",
            Conversion::new(31622400.0, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "sidereal_day",
            "time",
            Conversion::new(86164.0905, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "sidereal_month",
            "time",
            Conversion::new(2360591.5104, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "tropical_month",
            "time",
            Conversion::new(2360587.2, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "synodic_month",
            "time",
            Conversion::new(2551442.8, units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "planck_time",
            "time",
            Conversion::new(
                (1.054571817e-34 * 6.67430e-11 / 299792458.0_f64.powi(5)).sqrt(),
                units["second"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "degree_Celsius",
            "temperature",
            Conversion::new(1.0, units["kelvin"].clone()),
            units,
            dimensions
        );
        unit!(
            "degree_Rankine",
            "temperature",
            Conversion::new(5.0 / 9.0, units["kelvin"].clone()),
            units,
            dimensions
        );
        unit!(
            "degree_Fahrenheit",
            "temperature",
            Conversion::new(5.0 / 9.0, units["kelvin"].clone()),
            units,
            dimensions
        );
        unit!(
            "degree_Reaumur",
            "temperature",
            Conversion::new(4.0 / 5.0, units["kelvin"].clone()),
            units,
            dimensions
        );
        unit!(
            "atomic_unit_of_temperature",
            "temperature",
            Conversion::new(3.1577464e5, units["kelvin"].clone()),
            units,
            dimensions
        );
        unit!(
            "planck_temperature",
            "temperature",
            Conversion::new(
                (1.054571817e-34 * 299792458.0_f64.powi(5)
                    / (6.67430e-11 * 1.380649e-23_f64.powi(2)))
                .sqrt(),
                units["kelvin"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "are",
            "area",
            Conversion::new(100.0, units["meter"].clone().powi(2)),
            units,
            dimensions
        );
        unit!(
            "barn",
            "area",
            Conversion::new(1e-28, units["meter"].clone().powi(2)),
            units,
            dimensions
        );
        unit!(
            "darcy",
            "area",
            Conversion::new(9.869233e-13, units["meter"].clone().powi(2)),
            units,
            dimensions
        );
        unit!(
            "hectare",
            "area",
            Conversion::new(10000.0, units["meter"].clone().powi(2)),
            units,
            dimensions
        );

        unit!(
            "liter",
            "volume",
            Conversion::new(1e-3, units["meter"].clone().powi(3)),
            units,
            dimensions
        );
        unit!(
            "cubic_centimeter",
            "volume",
            Conversion::new(1e-6, units["meter"].clone().powi(3)),
            units,
            dimensions
        );
        unit!(
            "lambda",
            "volume",
            Conversion::new(1e-9, units["meter"].clone().powi(3)),
            units,
            dimensions
        );
        unit!(
            "stere",
            "volume",
            Conversion::new(1.0, units["meter"].clone().powi(3)),
            units,
            dimensions
        );

        unit!(
            "hertz",
            "frequency",
            Conversion::new(1.0, units["second"].clone().recip()),
            units,
            dimensions
        );
        unit!(
            "revolutions_per_minute",
            "frequency",
            Conversion::new(1.0 / 60.0, units["second"].clone().recip()),
            units,
            dimensions
        );
        unit!(
            "revolutions_per_second",
            "frequency",
            Conversion::new(1.0, units["second"].clone().recip()),
            units,
            dimensions
        );
        unit!(
            "counts_per_second",
            "frequency",
            Conversion::new(1.0, units["second"].clone().recip()),
            units,
            dimensions
        );

        unit!(
            "reciprocal_centimeter",
            "wavenumber",
            Conversion::new(1e2, units["meter"].clone().recip()),
            units,
            dimensions
        );

        unit!(
            "knot",
            "velocity",
            Conversion::new(
                1852.0 / 3600.0,
                units["meter"].clone() / units["second"].clone()
            ),
            units,
            dimensions
        );
        unit!(
            "mile_per_hour",
            "velocity",
            Conversion::new(
                1609.344 / 3600.0,
                units["meter"].clone() / units["second"].clone()
            ),
            units,
            dimensions
        );
        unit!(
            "kilometer_per_hour",
            "velocity",
            Conversion::new(
                1000.0 / 3600.0,
                units["meter"].clone() / units["second"].clone()
            ),
            units,
            dimensions
        );
        unit!(
            "kilometer_per_second",
            "velocity",
            Conversion::new(1000.0, units["meter"].clone() / units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "meter_per_second",
            "velocity",
            Conversion::new(1.0, units["meter"].clone() / units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "foot_per_second",
            "velocity",
            Conversion::new(0.3048, units["meter"].clone() / units["second"].clone()),
            units,
            dimensions
        );

        unit!(
            "sverdrup",
            "volumetric_flow_rate",
            Conversion::new(
                1e6,
                units["meter"].clone().powi(3) / units["second"].clone()
            ),
            units,
            dimensions
        );

        unit!(
            "galileo",
            "acceleration",
            Conversion::new(
                1e-2,
                units["meter"].clone() / units["second"].clone().powi(2)
            ),
            units,
            dimensions
        );

        unit!(
            "newton",
            "force",
            Conversion::new(
                1.0,
                units["kilogram"].clone() * units["meter"].clone()
                    / units["second"].clone().powi(2)
            ),
            units,
            dimensions
        );
        unit!(
            "dyne",
            "force",
            Conversion::new(1e-5, units["newton"].clone()),
            units,
            dimensions
        );
        unit!(
            "force_kilogram",
            "force",
            Conversion::new(9.80665, units["newton"].clone()),
            units,
            dimensions
        );
        unit!(
            "force_gram",
            "force",
            Conversion::new(9.80665e-3, units["newton"].clone()),
            units,
            dimensions
        );
        unit!(
            "force_metric_ton",
            "force",
            Conversion::new(9.80665e3, units["newton"].clone()),
            units,
            dimensions
        );
        unit!(
            "atomic_unit_of_force",
            "force",
            Conversion::new(8.23872336e-8, units["newton"].clone()),
            units,
            dimensions
        );

        unit!(
            "joule",
            "energy",
            Conversion::new(1.0, units["newton"].clone() * units["meter"].clone()),
            units,
            dimensions
        );
        unit!(
            "erg",
            "energy",
            Conversion::new(1e-7, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "watt_hour",
            "energy",
            Conversion::new(3600.0, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "electron_volt",
            "energy",
            Conversion::new(1.602176634e-19, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "rydberg",
            "energy",
            Conversion::new(2.1798723611035e-18, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "hartree",
            "energy",
            Conversion::new(4.3597447222071e-18, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "calorie",
            "energy",
            Conversion::new(4.184, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "international_calorie",
            "energy",
            Conversion::new(4.1868, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "fifteen_degree_calorie",
            "energy",
            Conversion::new(4.1855, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "british_thermal_unit",
            "energy",
            Conversion::new(1055.056, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "international_british_thermal_unit",
            "energy",
            Conversion::new(1055.05585262, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "thermochemical_british_thermal_unit",
            "energy",
            Conversion::new(1054.35026444, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "quadrillion_Btu",
            "energy",
            Conversion::new(1.055056e18, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "therm",
            "energy",
            Conversion::new(1.055056e8, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "US_therm",
            "energy",
            Conversion::new(1.054804e8, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "ton_TNT",
            "energy",
            Conversion::new(4.184e9, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "tonne_of_oil_equivalent",
            "energy",
            Conversion::new(4.1868e10, units["joule"].clone()),
            units,
            dimensions
        );
        unit!(
            "atmosphere_liter",
            "energy",
            Conversion::new(101.325, units["joule"].clone()),
            units,
            dimensions
        );

        unit!(
            "watt",
            "power",
            Conversion::new(1.0, units["joule"].clone() / units["second"].clone()),
            units,
            dimensions
        );
        unit!(
            "volt_ampere",
            "power",
            Conversion::new(1.0, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "horsepower",
            "power",
            Conversion::new(745.69987158227022, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "boiler_horsepower",
            "power",
            Conversion::new(9809.5, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "metric_horsepower",
            "power",
            Conversion::new(735.49875, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "electrical_horsepower",
            "power",
            Conversion::new(746.0, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "refrigeration_ton",
            "power",
            Conversion::new(3516.8528420667, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "cooling_tower_ton",
            "power",
            Conversion::new(4396.066052583375, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "standard_liter_per_minute",
            "power",
            Conversion::new(1.68875, units["watt"].clone()),
            units,
            dimensions
        );
        unit!(
            "conventional_watt_90",
            "power",
            Conversion::new(1.0000000000000002, units["watt"].clone()),
            units,
            dimensions
        );
    }
}

// pub static UNITS: Lazy<UnitRegistry> = Lazy::new(|| UnitRegistry::new());
