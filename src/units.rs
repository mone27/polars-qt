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

    pub fn to_scalar(&self) -> PolarsResult<Scalar> {
        let names: Series = self.units.iter().map(|u| u.name.clone()).collect();
        let powers: Series = self.units.iter().map(|u| Some(u.power)).collect();
        let ca_struct =
            StructChunked::from_series("unit".into(), names.len(), [names, powers].iter())?;
        Ok(Scalar::new(
            DataType::List(Box::new(DataType::Struct(vec![
                Field::new("name".into(), DataType::String),
                Field::new("power".into(), DataType::Int16),
            ]))),
            AnyValue::List(ca_struct.into_series()),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_to_scalar() {
        let units = Units {
            units: vec![
                Unit {
                    name: "m".to_string(),
                    power: 1,
                },
                Unit {
                    name: "s".to_string(),
                    power: 2,
                },
            ],
        };
        let scalar = units.to_scalar().unwrap();
        assert_eq!(
            scalar.dtype(),
            DataType::List(Box::new(DataType::Struct(vec![
                Field::new("name".into(), DataType::String),
                Field::new("power".into(), DataType::Int16),
            ])))
        );
        let units = Units::from_scalar(scalar).unwrap();
        assert_eq!(units.units.len(), 2);
        assert_eq!(units.units[0].name, "m");
        assert_eq!(units.units[0].power, 1);
        assert_eq!(units.units[1].name, "s");
        assert_eq!(units.units[1].power, 2);
    }
}
