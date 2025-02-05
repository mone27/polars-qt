use std::collections::HashMap;
use std::ops::{Div, Mul};

use num_rational::Rational64;
use polars::prelude::{polars_bail, PolarsResult};

// Other option

// we can express the relation between units (and dimensions) either as a nested graph of operations or as a tree of operations.
// or as a vec of (units, power) pairs and assume multiplication between them
// I actually think that the second data structure is better it is not nested
// just need to make sure that we input JSON is in this format (but that doesn't matter)
// so we are going for the second
// then we have an hashmap that can allows us to go from a unit to a unit with dimensions
// how to we name a unit without dimensions? maybe we don't need a name as the unit without dimension will only exist in polars
// the rust datastructure can habe both units and dimensions
// another problem  to consider is how are we going to express the relationi
// so this is a situation a single named unit can depend on multiple dimensions
// but then you also need to combine units
// but we need to ensure the
// now need to add to

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub struct Dimension {
    pub dimensions: Vec<(String, Rational64)>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BaseUnit {
    pub name: std::string::String, // e.g. meter
    // prefix: std::string::String, // e.g. kilo
    pub dimension: Dimension, // e.g. [length]
}

#[derive(Debug, PartialEq, Clone)]
pub struct Conversion {
    pub factor: f64,
    pub offset: Option<f64>,
    pub unit: Unit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    // pub units: Vec<(BaseUnit, Rational64)>,
    pub unit: BaseUnit,
    pub conversion: Box<Option<Conversion>>,
}

pub struct UnitRegistry {
    pub dimensions: HashMap<String, Dimension>,
    pub units: HashMap<String, Unit>,
}

impl Conversion {
    pub fn new(factor: f64, unit: Unit) -> Self {
        Self {
            factor,
            offset: None,
            unit,
        }
    }
}

impl BaseUnit {
    /// Simplify the unit by removing dimensions with a power of 0
    fn simplify(mut self) -> Self {
        self.dimension
            .dimensions
            .retain(|(_, power)| *power != Rational64::from_integer(0));
        self
    }
}

impl Mul for BaseUnit {
    type Output = BaseUnit;

    fn mul(self, rhs: BaseUnit) -> BaseUnit {
        let mut dimensions = self.dimension.dimensions.clone();
        for dim_rhs in &rhs.dimension.dimensions {
            if let Some(dim) = dimensions.iter_mut().find(|u| u.0 == dim_rhs.0) {
                dim.1 += dim_rhs.1;
            } else {
                dimensions.push(dim_rhs.clone());
            }
        }
        let unit = BaseUnit {
            name: format!("{} * {}", self.name, rhs.name),
            dimension: Dimension { dimensions },
        };
        unit.simplify()
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        let new_conversion = match (*self.conversion, *rhs.conversion) {
            (Some(conv1), Some(conv2)) => Some(Conversion {
                factor: conv1.factor * conv2.factor,
                offset: None,
                unit: conv1.unit * conv2.unit,
            }),
            (Some(conv), None) => Some(Conversion {
                factor: conv.factor,
                offset: None,
                unit: conv.unit
                    * Unit {
                        unit: rhs.unit.clone(),
                        conversion: Box::new(None),
                    },
            }),
            (None, Some(conv)) => Some(Conversion {
                factor: conv.factor,
                offset: None,
                unit: Unit {
                    unit: self.unit.clone(),
                    conversion: Box::new(None),
                } * conv.unit,
            }),
            (None, None) => None,
        };

        Unit {
            unit: self.unit * rhs.unit,
            conversion: Box::new(new_conversion),
        }
    }
}
impl Div for BaseUnit {
    type Output = BaseUnit;

    fn div(self, rhs: BaseUnit) -> BaseUnit {
        let mut dimensions = self.dimension.dimensions.clone();
        for dim_rhs in &rhs.dimension.dimensions {
            if let Some(dim) = dimensions.iter_mut().find(|u| u.0 == dim_rhs.0) {
                dim.1 -= dim_rhs.1;
            } else {
                dimensions.push((dim_rhs.0.clone(), -dim_rhs.1));
            }
        }
        let unit = BaseUnit {
            name: format!("{} / {}", self.name, rhs.name),
            dimension: Dimension { dimensions },
        };
        unit.simplify()
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Unit) -> Unit {
        let new_conversion = match (*self.conversion, *rhs.conversion) {
            (Some(conv1), Some(conv2)) => Some(Conversion {
                factor: conv1.factor / conv2.factor,
                offset: None,
                unit: conv1.unit / conv2.unit,
            }),
            (Some(conv), None) => Some(conv.clone()),
            (None, Some(conv)) => Some(Conversion {
                factor: 1.0 / conv.factor,
                offset: None,
                unit: conv.unit,
            }),
            (None, None) => None,
        };

        Unit {
            unit: self.unit / rhs.unit,
            conversion: Box::new(new_conversion),
        }
    }
}

impl UnitRegistry {
    pub fn convert(old_unit: Unit, new_unit: Unit) -> PolarsResult<f64> {
        let old_dim = &old_unit.unit.dimension;
        let new_dim = &new_unit.unit.dimension;
        if old_dim != new_dim {
            polars_bail!(ComputeError: "Cannot convert between units with different dimensions")
        }
        let old_conv = old_unit.conversion.as_ref();
        let new_conv = new_unit.conversion.as_ref();
        // either
        match (old_conv, new_conv) {
            (Some(old_conv), Some(new_conv)) => {
                if old_conv.unit == new_conv.unit {
                    assert!(
                        old_conv.offset.is_none() & new_conv.offset.is_none(),
                        "Offset not yet supported"
                    );
                    let factor = old_conv.factor / new_conv.factor;
                    Ok(factor)
                } else {
                    polars_bail!(ComputeError: "Cannot convert between units with different dimensions")
                }
            },
            (Some(old_conv), None) => {
                let factor = old_conv.factor;
                assert!(old_conv.offset.is_none());
                Ok(factor)
            },
            (None, Some(new_conv)) => {
                let factor = 1.0 / new_conv.factor;
                assert!(new_conv.offset.is_none());
                Ok(factor)
            },
            (None, None) => {
                if old_unit.unit == new_unit.unit {
                    Ok(1.0)
                } else {
                    polars_bail!(ComputeError: "Cannot convert between units with different dimensions")
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create common test units
    fn setup_length_units() -> (Unit, Unit, Unit) {
        let length = Dimension {
            dimensions: vec![("length".to_string(), Rational64::from_integer(1))],
        };

        let meter = Unit {
            unit: BaseUnit {
                name: "meter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(None),
        };

        let kilometer = Unit {
            unit: BaseUnit {
                name: "kilometer".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(Some(Conversion {
                factor: 1000.0,
                offset: None,
                unit: meter.clone(),
            })),
        };

        let centimeter = Unit {
            unit: BaseUnit {
                name: "centimeter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(Some(Conversion {
                factor: 0.01,
                offset: None,
                unit: meter.clone(),
            })),
        };

        (meter, kilometer, centimeter)
    }

    // Tests for Unit Multiplication
    #[test]
    fn test_unit_multiplication_basic() {
        let (meter, _, _) = setup_length_units();
        let result = meter.clone() * meter.clone();

        assert_eq!(
            result.unit.dimension.dimensions[0].1,
            Rational64::from_integer(2)
        );
        assert_eq!(result.unit.name, "meter * meter");
    }

    #[test]
    fn test_unit_multiplication_with_conversions() {
        let (meter, kilometer, _) = setup_length_units();
        let result = kilometer.clone() * kilometer.clone();
        let m2 = meter.clone() * meter.clone();

        assert!(result.conversion.is_some());
        if let Some(conv) = *result.conversion {
            assert_eq!(conv.factor, 1_000_000.0);
            assert_eq!(conv.unit, m2)
        }
    }

    #[test]
    fn test_unit_multiplication_mixed_conversion() {
        let (meter, kilometer, _) = setup_length_units();
        let result = meter.clone() * kilometer;
        let m2 = meter.clone() * meter.clone();

        assert!(result.conversion.is_some());
        if let Some(conv) = *result.conversion {
            assert_eq!(conv.factor, 1_000.0);
            assert_eq!(conv.unit, m2)
        }
    }

    // Tests for Unit Division
    #[test]
    fn test_unit_division_basic() {
        let (meter, _, _) = setup_length_units();
        let result = meter.clone() / meter.clone();

        assert!(result.unit.dimension.dimensions.is_empty());
    }

    #[test]
    fn test_unit_division_with_conversions() {
        let (meter, kilometer, _) = setup_length_units();
        let result = kilometer.clone() / kilometer.clone();
        let m_m = meter.clone() / meter.clone();

        assert!(result.conversion.is_some());
        if let Some(conv) = *result.conversion {
            assert_eq!(conv.factor, 1.0);
            assert_eq!(conv.unit, m_m)
        }
    }

    #[test]
    fn test_unit_division_mixed_conversion() {
        let (meter, kilometer, _) = setup_length_units();
        let result = kilometer / meter;

        assert!(result.conversion.is_some());
        if let Some(conv) = *result.conversion {
            assert_eq!(conv.factor, 1000.0);
        }
    }

    // Tests for Unit Conversion
    #[test]
    fn test_conversion_same_unit() {
        let (meter, _, _) = setup_length_units();
        let factor = UnitRegistry::convert(meter.clone(), meter.clone()).unwrap();
        assert_eq!(factor, 1.0);
    }

    #[test]
    fn test_conversion_to_larger_unit() {
        let (meter, kilometer, _) = setup_length_units();
        let factor = UnitRegistry::convert(meter, kilometer).unwrap();
        assert_eq!(factor, 0.001);
    }

    #[test]
    fn test_conversion_to_smaller_unit() {
        let (meter, _, centimeter) = setup_length_units();
        let factor = UnitRegistry::convert(meter, centimeter).unwrap();
        assert_eq!(factor, 100.0);
    }

    #[test]
    fn test_conversion_between_derived_units() {
        let (meter, kilometer, centimeter) = setup_length_units();
        let factor = UnitRegistry::convert(kilometer, centimeter).unwrap();
        assert_eq!(factor, 100_000.0);
    }

    #[test]
    #[should_panic(expected = "Cannot convert between units with different dimensions")]
    fn test_conversion_different_dimensions() {
        let (meter, _, _) = setup_length_units();
        let mass = Dimension {
            dimensions: vec![("mass".to_string(), Rational64::from_integer(1))],
        };

        let kilogram = Unit {
            unit: BaseUnit {
                name: "kilogram".to_string(),
                dimension: mass,
            },
            conversion: Box::new(None),
        };

        UnitRegistry::convert(meter, kilogram).unwrap();
    }

    #[test]
    #[should_panic(expected = "Offset not yet supported")]
    fn test_conversion_with_offset() {
        let length = Dimension {
            dimensions: vec![("length".to_string(), Rational64::from_integer(1))],
        };

        let base_meter = Unit {
            unit: BaseUnit {
                name: "meter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(None),
        };

        let meter_with_offset = Unit {
            unit: BaseUnit {
                name: "meter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(Some(Conversion {
                factor: 1.0,
                offset: Some(10.0),
                unit: base_meter.clone(),
            })),
        };

        let _ = UnitRegistry::convert(meter_with_offset.clone(), meter_with_offset.clone());
    }
    // Additional edge cases and complex scenarios
    #[test]
    fn test_complex_dimension_multiplication() {
        let dim = Dimension {
            dimensions: vec![
                ("length".to_string(), Rational64::from_integer(1)),
                ("time".to_string(), Rational64::from_integer(-2)),
            ],
        };

        let unit1 = Unit {
            unit: BaseUnit {
                name: "unit1".to_string(),
                dimension: dim.clone(),
            },
            conversion: Box::new(None),
        };

        let result = unit1.clone() * unit1.clone();
        assert_eq!(
            result.unit.dimension.dimensions[0].1,
            Rational64::from_integer(2)
        );
        assert_eq!(
            result.unit.dimension.dimensions[1].1,
            Rational64::from_integer(-4)
        );
    }
}
