use polars::prelude::*;

pub struct Unit {
    name: std::string::String,
    power: i16,
}

pub struct Units {
    units: Vec<Unit>,
}

// impl Unit {
//     pub fn from_struct(x: AnyValue) -> PolarsResult<Self> {
//         if let AnyValue::StructOwned(str_owned) = x {
//             let (values, fields) = *str_owned;
//             Self::from_unit_values(&values)
//         } else {
//             polars_bail!(InvalidOperation: "Expected StructOwned, got {:?}", x)
//         }
//     }

//     fn from_unit_values(values: &[AnyValue]) -> PolarsResult<Unit> {
//         if let (AnyValue::String(name), AnyValue::Int16(power)) = (&values[0], &values[1]) {
//             Ok(Unit {
//                 name: name.to_string(),
//                 power: *power,
//             })
//         } else {
//             polars_bail!(ComputeError: "Invalid Unit values. Expected 'name' as String and 'power' as Int16, got {:?}", values);
//         }
//     }
// }

impl Units {
    pub fn from_scalar(scalar: Scalar) -> PolarsResult<Self> {
        if let AnyValue::List(list) = scalar.value() {
            let ca_units: &StructChunked = list.struct_()?;
            if ca_units.has_nulls() {
                polars_bail!(ComputeError: "Unit cannot have null values");
            }
            // check that the dtype is correct
            Self::check_unit_fields(ca_units.struct_fields())?;
            let fields = ca_units.fields_as_series();
            let (names, powers) = (fields[0].str()?, fields[1].i16()?);
            let units = names
                .iter()
                .zip(powers)
                .map(|(name, power)| Unit {
                    name: name.unwrap().to_string(), // safe to unwrap because we checked for nulls
                    power: power.unwrap(),
                })
                .collect::<Vec<Unit>>();
            Ok(Self { units })
        } else {
            polars_bail!(ComputeError: "Expected List of Structs, got {:?}", scalar.dtype());
        }
    }
    fn check_unit_fields(fields: &[Field]) -> PolarsResult<()> {
        if fields.len() != 2 {
            polars_bail!(ComputeError: "Unit struct must have 2 fields, got {:?}", fields.len());
        }
        let name_field = fields.get(0).unwrap();
        let power_field = fields.get(1).unwrap();
        if name_field.name() == "name"
            && name_field.dtype() == &DataType::String
            && power_field.name() == "power"
            && power_field.dtype() == &DataType::Int16
        {
            return Ok(());
        }
        polars_bail!(ComputeError: "Invalid Unit struct. Expected fields 'name' and 'power' with types String and Int16, got {:?}", fields);
    }
}
