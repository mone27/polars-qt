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

macro_rules! unit {
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
        unit!(units, "meter", dimensions["length"]);
        unit!(units, "kilogram", dimensions["mass"]);
        unit!(units, "second", dimensions["time"]);
        unit!(units, "ampere", dimensions["current"]);
        unit!(units, "kelvin", dimensions["temperature"]);
        unit!(units, "mole", dimensions["amount"]);
        unit!(units, "candela", dimensions["luminosity"]);
        unit!(units, "radian", dimensions["dimensionless"]);
        unit!(units, "bit", dimensions["dimensionless"]);
        unit!(units, "count", dimensions["dimensionless"]);
    }

    fn add_derived(units: &mut HashMap<String, Unit>, dimensions: &HashMap<String, Dimension>) {
        units.insert(
            "turn".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "turn".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 2.0 * std::f64::consts::PI,
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "degree".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "degree".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 2.0 * std::f64::consts::PI / 360.0,
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "arcminute".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "arcminute".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: std::f64::consts::PI / (180.0 * 60.0),
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "arcsecond".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "arcsecond".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: std::f64::consts::PI / (180.0 * 60.0 * 60.0),
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "milliarcsecond".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "milliarcsecond".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: std::f64::consts::PI / (180.0 * 60.0 * 60.0 * 1000.0),
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "grade".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "grade".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: std::f64::consts::PI / 200.0,
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "mil".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "mil".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: std::f64::consts::PI / 3200.0,
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "steradian".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "steradian".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1.0,
                    offset: None,
                    unit: units["radian"].clone(),
                })),
            },
        );

        units.insert(
            "square_degree".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "square_degree".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: (std::f64::consts::PI / 180.0).powi(2),
                    offset: None,
                    unit: units["steradian"].clone(),
                })),
            },
        );

        units.insert(
            "baud".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "baud".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1.0,
                    offset: None,
                    unit: units["bit"].clone(),
                })),
            },
        );

        units.insert(
            "byte".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "byte".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 8.0,
                    offset: None,
                    unit: units["bit"].clone(),
                })),
            },
        );

        units.insert(
            "percent".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "percent".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 0.01,
                    offset: None,
                    unit: units["dimensionless"].clone(),
                })),
            },
        );

        units.insert(
            "permille".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "permille".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 0.001,
                    offset: None,
                    unit: units["dimensionless"].clone(),
                })),
            },
        );

        units.insert(
            "ppm".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "ppm".to_string(),
                    dimension: dimensions["dimensionless"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1e-6,
                    offset: None,
                    unit: units["dimensionless"].clone(),
                })),
            },
        );

        units.insert(
            "angstrom".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "angstrom".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1e-10,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "micron".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "micron".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1e-6,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "fermi".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "fermi".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1e-15,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "light_year".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "light_year".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 9.4607e15,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "astronomical_unit".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "astronomical_unit".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1.495978707e11,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "parsec".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "parsec".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 3.085677581e16,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "nautical_mile".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "nautical_mile".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1852.0,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "bohr".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "bohr".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 5.29177210903e-11,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );

        units.insert(
            "planck_length".to_string(),
            Unit {
                unit: BaseUnit {
                    name: "planck_length".to_string(),
                    dimension: dimensions["length"].clone(),
                },
                conversion: Box::new(Some(Conversion {
                    factor: 1.616255e-35,
                    offset: None,
                    unit: units["meter"].clone(),
                })),
            },
        );
    }
}

// pub static UNITS: Lazy<UnitRegistry> = Lazy::new(|| UnitRegistry::new());
