use num_rational::Rational64;
use pyo3_polars::export::polars_core::utils::rayon::vec;

// struct BaseDimension{
//     name: String
// }


// enum Operator{
//     Mul,
//     Div,
//     Exp,
//     Noop
// }

// enum Operand{
//     BaseDimension(Dimension),
//     Dimension(Dimension),
//     Number(Rational64),
//     None
// }

// struct Operation{
//     operator: Operator,
//     left: Operand,
//     right: Operand
// }


// struct Dimension{
//     // name: String, 
//     operands: Box<Operation> // Needed Box because is recursive
// }

// struct BaseUnit{
//     name: String,
//     dimension: Dimension
// }

// struct Unit{
//     name: String,
//     dimension: Dimension,
     
// }



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

struct SingleDimension{
    name: std::string::String,
}

struct Dimension{
    dimensions: Vec<(SingleDimension, Rational64)>
}

struct SingleUnit{
    name: std::string::String,
    dimension: Dimension
}

struct conversion{
    factor: f64,
    unit: Unit
}


struct Unit{
    units: Vec<(SingleUnit, Rational64)>,
    conversion: Box<Option<conversion>>
}
