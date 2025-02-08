use std::collections::HashMap;
use std::ops::{Div, Mul};

use anyhow::{anyhow, bail, Context, Result};
use num_rational::Rational64;

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
pub struct SimpleUnit {
    pub name: std::string::String, // e.g. meter
    pub dimension: Dimension,      // e.g. [length]
}

#[derive(Debug, PartialEq, Clone)]
pub struct Conversion {
    pub factor: f64,
    pub offset: Option<f64>,
    pub base_unit: Unit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    pub simple_unit: SimpleUnit,
    pub conversion: Box<Option<Conversion>>,
}

impl Conversion {
    pub fn new(factor: f64, unit: Unit) -> Self {
        Self {
            factor,
            offset: None,
            base_unit: unit,
        }
    }
}

impl SimpleUnit {
    /// Simplify the unit by removing dimensions with a power of 0
    fn simplify(mut self) -> Self {
        self.dimension
            .dimensions
            .retain(|(_, power)| *power != Rational64::from_integer(0));
        self
    }
}

impl Mul for SimpleUnit {
    type Output = SimpleUnit;

    fn mul(self, rhs: SimpleUnit) -> SimpleUnit {
        let mut dimensions = self.dimension.dimensions.clone();
        for dim_rhs in &rhs.dimension.dimensions {
            if let Some(dim) = dimensions.iter_mut().find(|u| u.0 == dim_rhs.0) {
                dim.1 += dim_rhs.1;
            } else {
                dimensions.push(dim_rhs.clone());
            }
        }
        let unit = SimpleUnit {
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
                base_unit: conv1.base_unit * conv2.base_unit,
            }),
            (Some(conv), None) => Some(Conversion {
                factor: conv.factor,
                offset: None,
                base_unit: conv.base_unit
                    * Unit {
                        simple_unit: rhs.simple_unit.clone(),
                        conversion: Box::new(None),
                    },
            }),
            (None, Some(conv)) => Some(Conversion {
                factor: conv.factor,
                offset: None,
                base_unit: Unit {
                    simple_unit: self.simple_unit.clone(),
                    conversion: Box::new(None),
                } * conv.base_unit,
            }),
            (None, None) => None,
        };

        Unit {
            simple_unit: self.simple_unit * rhs.simple_unit,
            conversion: Box::new(new_conversion),
        }
    }
}
impl Div for SimpleUnit {
    type Output = SimpleUnit;

    fn div(self, rhs: SimpleUnit) -> SimpleUnit {
        let mut dimensions = self.dimension.dimensions.clone();
        for dim_rhs in &rhs.dimension.dimensions {
            if let Some(dim) = dimensions.iter_mut().find(|u| u.0 == dim_rhs.0) {
                dim.1 -= dim_rhs.1;
            } else {
                dimensions.push((dim_rhs.0.clone(), -dim_rhs.1));
            }
        }
        let unit = SimpleUnit {
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
                base_unit: conv1.base_unit / conv2.base_unit,
            }),
            (Some(conv), None) => Some(conv.clone()),
            (None, Some(conv)) => Some(Conversion {
                factor: 1.0 / conv.factor,
                offset: None,
                base_unit: conv.base_unit,
            }),
            (None, None) => None,
        };

        Unit {
            simple_unit: self.simple_unit / rhs.simple_unit,
            conversion: Box::new(new_conversion),
        }
    }
}

impl Dimension {
    pub fn pow(&self, exp: i64) -> Self {
        let mut new_dimensions = self.dimensions.clone();
        for dim in &mut new_dimensions {
            dim.1 *= Rational64::from_integer(exp);
        }
        Self {
            dimensions: new_dimensions,
        }
    }
}

impl SimpleUnit {
    pub fn pow(&self, exp: i64) -> Self {
        SimpleUnit {
            name: format!("{}^{}", self.name, exp),
            dimension: self.dimension.pow(exp),
        }
    }
}

impl Unit {
    pub fn with_name(&self, name: &str) -> Self {
        Self {
            simple_unit: SimpleUnit {
                name: name.to_string(),
                dimension: self.simple_unit.dimension.clone(),
            },
            conversion: self.conversion.clone(),
        }
    }

    pub fn pow(&self, exp: i64) -> Self {
        let new_conversion = match *self.conversion {
            Some(ref conv) => Some(Conversion {
                factor: conv.factor.powi(exp as i32),
                offset: None,
                base_unit: conv.base_unit.clone().pow(exp),
            }),
            None => None,
        };
        Self {
            simple_unit: self.simple_unit.pow(exp),
            conversion: Box::new(new_conversion),
        }
    }
}

pub struct UnitRegistry {
    pub dimensions: HashMap<String, Dimension>,
    pub units: HashMap<String, Unit>,
}
impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            dimensions: HashMap::new(),
            units: HashMap::new(),
        }
    }

    pub fn get_dimension(&self, name: &str) -> Result<Dimension> {
        self.dimensions
            .get(name)
            .cloned()
            .context(format!("dimension {} not found", name))
    }

    pub fn get_unit(&self, name: &str) -> Result<Unit> {
        self.units
            .get(name)
            .cloned()
            .context(format!("unit {} not found", name))
    }

    pub fn try_get_dimension(&self, name: &str) -> Dimension {
        self.get_dimension(name).unwrap()
    }

    pub fn try_get_unit(&self, name: &str) -> Unit {
        self.get_unit(name).unwrap()
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.units.insert(unit.simple_unit.name.clone(), unit);
    }

    pub fn add_unit_simple(&mut self, name: &str, dimension: &str) {
        let unit = Unit {
            simple_unit: SimpleUnit {
                name: name.to_string(),
                dimension: self.try_get_dimension(dimension),
            },
            conversion: Box::new(None),
        };
        self.add_unit(unit);
    }

    pub fn add_unit_deriv(&mut self, name: &str, dimension: &str, conv_factor: f64, conv_base_name: &str) {
        self.add_unit_deriv_offset(name, dimension, conv_factor, None, conv_base_name);
    }

    pub fn add_unit_deriv_offset(
        &mut self,
        name: &str,
        dimension: &str,
        conv_factor: f64,
        conv_offset: Option<f64>,
        conv_base_name: &str,
    ) {
        let unit = Unit {
            simple_unit: SimpleUnit {
                name: name.to_string(),
                dimension: self.try_get_dimension(dimension),
            },
            conversion: Box::new(Some(Conversion {
                factor: conv_factor,
                offset: conv_offset,
                base_unit: self.try_get_unit(conv_base_name),
            })),
        };
        self.add_unit(unit);
    }

    pub fn add_dimension(&mut self, name: &str, dimension: Dimension) {
        self.dimensions.insert(name.to_string(), dimension);
    }

    pub fn add_dimension_simple(&mut self, name: &str) {
        let dimension = Dimension::new_simple(name);
        self.add_dimension(name, dimension);
    }
    pub fn convert_units(old_unit: Unit, new_unit: Unit) -> Result<f64> {
        let old_dim = &old_unit.simple_unit.dimension;
        let new_dim = &new_unit.simple_unit.dimension;
        if old_dim != new_dim {
            bail!(
                "Cannot convert between units with different dimensions, got {:?} and {:?}",
                old_dim,
                new_dim
            );
        }
        let old_conv = old_unit.conversion.as_ref();
        let new_conv = new_unit.conversion.as_ref();
        match (old_conv, new_conv) {
            (Some(old_conv), Some(new_conv)) => {
                if old_conv.base_unit == new_conv.base_unit {
                    assert!(
                        old_conv.offset.is_none() & new_conv.offset.is_none(),
                        "Offset not yet supported"
                    );
                    let factor = old_conv.factor / new_conv.factor;
                    Ok(factor)
                } else {
                    bail!("Cannot convert between units with different dimensions");
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
                if old_unit.simple_unit == new_unit.simple_unit {
                    Ok(1.0)
                } else {
                    Err(anyhow!(
                        "Cannot convert between units with different dimensions, got {:?} {:?}",
                        old_unit.simple_unit,
                        new_unit.simple_unit
                    ))
                }
            },
        }
    }

    pub fn convert(&self, unit_from: String, unit_to: String) -> Result<f64> {
        let unit_from = self.units.get(&unit_from).unwrap();
        let unit_to = self.units.get(&unit_to).unwrap();
        Self::convert_units(unit_from.clone(), unit_to.clone())
    }
}

impl Dimension {
    pub fn new_simple(name: &str) -> Self {
        Self {
            dimensions: vec![(name.to_string(), Rational64::from_integer(1))],
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
            simple_unit: SimpleUnit {
                name: "meter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(None),
        };

        let kilometer = Unit {
            simple_unit: SimpleUnit {
                name: "kilometer".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(Some(Conversion {
                factor: 1000.0,
                offset: None,
                base_unit: meter.clone(),
            })),
        };

        let centimeter = Unit {
            simple_unit: SimpleUnit {
                name: "centimeter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(Some(Conversion {
                factor: 0.01,
                offset: None,
                base_unit: meter.clone(),
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
            result.simple_unit.dimension.dimensions[0].1,
            Rational64::from_integer(2)
        );
        assert_eq!(result.simple_unit.name, "meter * meter");
    }

    #[test]
    fn test_unit_multiplication_with_conversions() {
        let (meter, kilometer, _) = setup_length_units();
        let result = kilometer.clone() * kilometer.clone();
        let m2 = meter.clone() * meter.clone();

        assert!(result.conversion.is_some());
        if let Some(conv) = *result.conversion {
            assert_eq!(conv.factor, 1_000_000.0);
            assert_eq!(conv.base_unit, m2)
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
            assert_eq!(conv.base_unit, m2)
        }
    }

    // Tests for Unit Division
    #[test]
    fn test_unit_division_basic() {
        let (meter, _, _) = setup_length_units();
        let result = meter.clone() / meter.clone();

        assert!(result.simple_unit.dimension.dimensions.is_empty());
    }

    #[test]
    fn test_unit_division_with_conversions() {
        let (meter, kilometer, _) = setup_length_units();
        let result = kilometer.clone() / kilometer.clone();
        let m_m = meter.clone() / meter.clone();

        assert!(result.conversion.is_some());
        if let Some(conv) = *result.conversion {
            assert_eq!(conv.factor, 1.0);
            assert_eq!(conv.base_unit, m_m)
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

    #[test]
    fn test_unit_pow_positive_integer() {
        let (meter, _, _) = setup_length_units();
        let result = meter.clone().pow(3);

        assert_eq!(
            result.simple_unit.dimension.dimensions[0].1,
            Rational64::from_integer(3)
        );
        assert_eq!(result.simple_unit.name, "meter^3");
    }

    #[test]
    fn test_unit_pow_negative() {
        let (meter, _, _) = setup_length_units();
        let result = meter.clone().pow(-3);

        assert_eq!(
            result.simple_unit.dimension.dimensions[0].1,
            Rational64::from_integer(-3)
        );
        assert_eq!(result.simple_unit.name, "meter^-3");
    }

    // Tests for Unit Conversion
    #[test]
    fn test_conversion_same_unit() {
        let (meter, _, _) = setup_length_units();
        let factor = UnitRegistry::convert_units(meter.clone(), meter.clone()).unwrap();
        assert_eq!(factor, 1.0);
    }

    #[test]
    fn test_conversion_to_larger_unit() {
        let (meter, kilometer, _) = setup_length_units();
        let factor = UnitRegistry::convert_units(meter, kilometer).unwrap();
        assert_eq!(factor, 0.001);
    }

    #[test]
    fn test_conversion_to_smaller_unit() {
        let (meter, _, centimeter) = setup_length_units();
        let factor = UnitRegistry::convert_units(meter, centimeter).unwrap();
        assert_eq!(factor, 100.0);
    }

    #[test]
    fn test_conversion_between_derived_units() {
        let (_meter, kilometer, centimeter) = setup_length_units();
        let factor = UnitRegistry::convert_units(kilometer, centimeter).unwrap();
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
            simple_unit: SimpleUnit {
                name: "kilogram".to_string(),
                dimension: mass,
            },
            conversion: Box::new(None),
        };

        UnitRegistry::convert_units(meter, kilogram).unwrap();
    }

    #[test]
    #[should_panic(expected = "Offset not yet supported")]
    fn test_conversion_with_offset() {
        let length = Dimension {
            dimensions: vec![("length".to_string(), Rational64::from_integer(1))],
        };

        let base_meter = Unit {
            simple_unit: SimpleUnit {
                name: "meter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(None),
        };

        let meter_with_offset = Unit {
            simple_unit: SimpleUnit {
                name: "meter".to_string(),
                dimension: length.clone(),
            },
            conversion: Box::new(Some(Conversion {
                factor: 1.0,
                offset: Some(10.0),
                base_unit: base_meter.clone(),
            })),
        };

        let _ = UnitRegistry::convert_units(meter_with_offset.clone(), meter_with_offset.clone());
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
            simple_unit: SimpleUnit {
                name: "unit1".to_string(),
                dimension: dim.clone(),
            },
            conversion: Box::new(None),
        };

        let result = unit1.clone() * unit1.clone();
        assert_eq!(
            result.simple_unit.dimension.dimensions[0].1,
            Rational64::from_integer(2)
        );
        assert_eq!(
            result.simple_unit.dimension.dimensions[1].1,
            Rational64::from_integer(-4)
        );
    }
}
