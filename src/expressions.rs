#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

use crate::expressions::polars_plan::prelude::Expr;

// struct QuantityChuncked<T>
// where
//     T: PolarsNumericType,
//     ChunkedArray<T>: IntoSeries,
// {
//     value: ChunkedArray<T>,
//     unit: ChunkedArray<StringType>,
//     name: PlSmallStr,
// }

// impl<T> QuantityChuncked<T>
// where
//     ChunkedArray<T>: IntoSeries,
//     T: PolarsNumericType,
// {
//     fn from_series(series: &Series) -> PolarsResult<Self> {
//         let quantity_array = series.struct_()?;
//         is_valid_unit_dtype(quantity_array.dtype())?;
//         let fields = &quantity_array.fields_as_series();
//         let (value, unit) = (fields.get(0).unwrap(), fields.get(1).unwrap());
//         Ok(Self {
//             value: value.unpack::<T>().unwrap().clone(),
//             unit: unit.str().unwrap().clone(),
//             name: series.name().clone(),
//         })
//     }

//     fn to_series(&self) -> PolarsResult<Series> {
//         let fields = vec![
//             self.value.clone().into_series(),
//             self.unit.clone().into_series(),
//         ];
//         Ok(
//             StructChunked::from_series(self.name.clone(), self.value.len(), fields.iter())?
//                 .into_series(),
//         )
//     }
// }

// impl<'a, T> QuantityChuncked<'a, T>
// where
//     T: PolarsNumericType,
// {
//     fn from_series(series: &'a Series) -> PolarsResult<Self> {
//         let quantity_array = series.struct_()?;
//         is_valid_unit_dtype(quantity_array.dtype())?;
//         let fields = quantity_array.fields_as_series();

//         let value = fields.get(0).unwrap().unpack::<T>()?;
//         let unit = fields.get(1).unwrap().utf8()?;

//         Ok(Self {
//             value,
//             unit,
//         })
//     }
// }

fn check_valid_unit_dtype(dtype: &DataType) -> PolarsResult<bool> {
    match dtype {
        DataType::Struct(fields) => {
            if let (Some(value_field), Some(unit_field)) = (fields.get(0), fields.get(1)) {
                if value_field.name == "value"
                    && value_field.dtype.is_numeric()
                    && unit_field.name == "unit"
                    && unit_field.dtype == DataType::String
                {
                    Ok(true)
                } else {
                    polars_bail!(InvalidOperation: "Invalid Unit. Expected struct with fields 'value' and 'unit' and types numeric and String, got {:?}", fields)
                }
            } else {
                polars_bail!(InvalidOperation: "Invalid Unit. Expected struct with 2 fields ('value' and 'unit'), got {:?} fields ({:?})", fields.len(), fields.into_iter().map(|f| f.name.clone()).collect::<Vec<_>>())
            }
        },
        dtype => polars_bail!(InvalidOperation: "Expected Struct dtype, got {}", dtype),
    }
}

fn is_all_same<T: PartialEq>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] == w[1])
}

fn unit_output(input_fields: &[Field]) -> PolarsResult<Field> {
    let dtypes: Vec<&DataType> = input_fields.iter().map(|f| f.dtype()).collect();
    // simplying assumptions: the value dtype must be the same for all inputs
    if !is_all_same(&dtypes) {
        polars_bail!(InvalidOperation: "Expected all input fields to have the same dtype, got {:?}", dtypes)
    }
    // check that datatype is valid
    check_valid_unit_dtype(dtypes[0])?;
    Ok(Field::new("unit".into(), DataType::String))
}

// #[polars_expr(output_type_func=unit_output)]
// fn noop(inputs: &[Series]) -> PolarsResult<Series> {
//     let struct_ = inputs[0].struct_()?;
//     let fields = struct_.fields_as_series();

//     if fields.is_empty() {
//         return Ok(inputs[0].clone());
//     }

//     let fields = fields
//         .iter()
//         .map(|s| {
//             let s = s.clone();
//             println!("{:?}", s);
//             s
//         })
//         .collect::<Vec<_>>();

//     StructChunked::from_series(struct_.name().clone(), struct_.len(), fields.iter())
//         .map(|ca| ca.into_series())
// }

#[polars_expr(output_type_func=unit_output)]
fn abs(inputs: &[Series]) -> PolarsResult<Series> {
    apply_unary(&inputs[0], col("value").abs())
}

#[polars_expr(output_type_func=unit_output)]
fn noop(inputs: &[Series]) -> PolarsResult<Series> {
    apply_unary(&inputs[0], col("value"))
}

fn check_same_unit(ca: &StringChunked) -> PolarsResult<()> {
    let mut iter = ca.iter();
    let first = iter.next().unwrap();
    if iter.all(|s| s == first) {
        Ok(())
    } else {
        polars_bail!(InvalidOperation: "Expected all units to be the same, got {:?}", ca.unique())
    }
    }

fn apply_unary(input: &Series, expr: Expr) -> PolarsResult<Series> {
    let ca = input.struct_()?;
    check_valid_unit_dtype(ca.dtype())?;
    let fields = &ca.fields_as_series();
    let (value, unit) = (
        fields.get(0).unwrap(),
        fields.get(1).unwrap().clone(),
    );
    check_same_unit(unit.str()?)?;
    let df = df!["value" => value]?.lazy().select(&[expr]).collect()?;
    let value = df["value"].clone();
    let fields = vec![value.as_series().unwrap().clone(), unit.into_series()];
    Ok(StructChunked::from_series(input.name().clone(), input.len(), fields.iter())?.into_series())
}

// fn apply_unary<F, T>(inputs: &[Series], func: F) -> PolarsResult<Series> where F: Fn(T) -> T, T: PolarsNumericType {
//     let quantity = inputs[0].struct_()?;
//     is_valid_unit_dtype(quantity.dtype())?;
//     let value = quantity.field_by_name("value")?;
//     let unit = quantity.field_by_name("unit")?;

// }
