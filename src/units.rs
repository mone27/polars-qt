use polars::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    name: std::string::String,
    power: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Units {
    units: Vec<Unit>,
}

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

    pub fn check_valid_unit_dtype(dtype: &DataType) -> PolarsResult<()> {
        if let DataType::List(inner) = dtype {
            let inner = inner.as_ref();
            if let DataType::Struct(fields) = inner.as_ref() {
                Self::check_unit_fields(&fields)
            }
            else {
                polars_bail!(ComputeError: "Invalid Unit dtype: expected List of Struct, got List of {:?}", inner)
            }
        }
        else {
            polars_bail!(ComputeError: "Invalid Unit dtype: expected List, got {:?}", dtype)
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
        let ca_struct = StructChunked::from_series(
            "unit".into(),
            names.len(),
            [
                names.with_name("name".into()),
                powers.with_name("power".into()),
            ]
            .iter(),
        )?;
        Ok(Scalar::new(
            DataType::List(Box::new(DataType::Struct(vec![
                Field::new("name".into(), DataType::String),
                Field::new("power".into(), DataType::Int16),
            ]))),
            AnyValue::List(ca_struct.into_series()),
        ))
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let mut units = self.units.clone();
        for unit_other in &other.units {
            if let Some(u) = units.iter_mut().find(|u| u.name == unit_other.name) {
                u.power += unit_other.power;
            } else {
                units.push(unit_other.clone());
            }
        }
        Self { units }
    }

    pub fn divide(&self, other: &Self) -> Self {
        let mut units = self.units.clone();
        for unit_other in &other.units {
            if let Some(u) = units.iter_mut().find(|u| u.name == unit_other.name) {
                u.power -= unit_other.power;
            } else {
                units.push(Unit {
                    name: unit_other.name.clone(),
                    power: -unit_other.power,
                });
            }
        }
        Self { units }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
        let units = Units::from_scalar(scalar).unwrap();
        assert_eq!(units.units.len(), 2);
        assert_eq!(units.units[0].name, "m");
        assert_eq!(units.units[0].power, 1);
        assert_eq!(units.units[1].name, "s");
        assert_eq!(units.units[1].power, 2);
    }

    #[test]
    fn test_empty_units() {
        let units = Units { units: vec![] };
        let scalar = units.to_scalar().unwrap();
        let units = Units::from_scalar(scalar).unwrap();
        assert_eq!(units.units.len(), 0);
    }

    #[test]
    fn test_invalid_scalar() {
        let invalid_scalar = Scalar::new(DataType::Int32, AnyValue::Int32(42));
        let result = Units::from_scalar(invalid_scalar);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiply() {
        let units1 = Units {
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
        let units2 = Units {
            units: vec![
                Unit {
                    name: "m".to_string(),
                    power: 2,
                },
                Unit {
                    name: "s".to_string(),
                    power: 3,
                },
            ],
        };
        let units = units1.multiply(&units2);
        assert_eq!(units.units.len(), 2);
        assert_eq!(units.units[0].name, "m");
        assert_eq!(units.units[0].power, 3);
        assert_eq!(units.units[1].name, "s");
        assert_eq!(units.units[1].power, 5);
    }

    #[test]
    fn test_divide() {
        let units1 = Units {
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
        let units2 = Units {
            units: vec![
                Unit {
                    name: "m".to_string(),
                    power: 2,
                },
                Unit {
                    name: "s".to_string(),
                    power: 3,
                },
            ],
        };
        let units = units1.divide(&units2);
        assert_eq!(units.units.len(), 2);
        assert_eq!(units.units[0].name, "m");
        assert_eq!(units.units[0].power, -1);
        assert_eq!(units.units[1].name, "s");
        assert_eq!(units.units[1].power, -1);
    }
}
