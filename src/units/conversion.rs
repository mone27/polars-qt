use std::collections::{HashMap, HashSet};

use num_rational::Rational64;
use pyo3_polars::export::polars_core::utils::rayon::vec;

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
