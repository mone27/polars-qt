use std::collections::HashMap;

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

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        let new_dimension = Dimension {
            dimensions: self
                .unit
                .dimension
                .dimensions
                .iter()
                .chain(rhs.unit.dimension.dimensions.iter())
                .map(|(name, exp)| (name.clone(), *exp))
                .collect(),
        };

        let new_conversion = match (*self.conversion, *rhs.conversion) {
            (Some(conv1), Some(conv2)) => Some(Conversion {
                factor: conv1.factor * conv2.factor,
                offset: None,
                unit: conv1.unit.clone(),
            }),
            (Some(conv), None) | (None, Some(conv)) => Some(conv.clone()),
            (None, None) => None,
        };

        Unit {
            unit: BaseUnit {
                name: format!("({} * {})", self.unit.name, rhs.unit.name),
                dimension: new_dimension,
            },
            conversion: Box::new(new_conversion),
        }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Unit) -> Unit {
        let new_dimension = Dimension {
            dimensions: self
                .unit
                .dimension
                .dimensions
                .iter()
                .chain(
                    rhs.unit
                        .dimension
                        .dimensions
                        .iter()
                        .map(|(name, exp)| (name.clone(), -exp)),
                )
                .map(|(name, exp)| (name.clone(), *exp))
                .collect(),
        };

        let new_conversion = match (*self.conversion, *rhs.conversion) {
            (Some(conv1), Some(conv2)) => Some(Conversion {
                factor: conv1.factor / conv2.factor,
                offset: None,
                unit: conv1.unit.clone(),
            }),
            (Some(conv), None) => Some(conv.clone()),
            (None, Some(conv)) => Some(Conversion {
                factor: 1.0 / conv.factor,
                offset: None,
                unit: conv.unit.clone(),
            }),
            (None, None) => None,
        };

        Unit {
            unit: BaseUnit {
                name: format!("({} / {})", self.unit.name, rhs.unit.name),
                dimension: new_dimension,
            },
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
mod test {

    use super::*;
    use crate::units::definitions;

    #[test]
    fn test_conversion_base() {
        let mut registry = UnitRegistry {
            dimensions: HashMap::new(),
            units: HashMap::new(),
        };

        let lenght = Dimension {
            dimensions: vec![("length".to_string(), Rational64::from_integer(1))],
        };

        let meter = Unit {
            unit: BaseUnit {
                name: "meter".to_string(),
                dimension: lenght.clone(),
            },
            conversion: Box::new(None),
        };
        let kilometer = Unit {
            unit: BaseUnit {
                name: "kilometer".to_string(),
                dimension: lenght.clone(),
            },
            conversion: Box::new(Some(Conversion {
                factor: 1000.0,
                offset: None,
                unit: meter.clone(),
            })),
        };

        let factor = UnitRegistry::convert(meter, kilometer).unwrap();
        assert_eq!(factor, 0.001);
    }
}
