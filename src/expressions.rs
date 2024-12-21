#![allow(clippy::unused_unit)]
use polars::prelude::*;
use polars_core::utils::arrow::legacy::is_valid;
use pyo3_polars::derive::polars_expr;

struct QuantityChuncked<T>
where
    T: PolarsNumericType,
    SeriesWrap<polars::prelude::ChunkedArray<T>>: polars::prelude::SeriesTrait
{
    value: ChunkedArray<T>,
    unit: ChunkedArray<StringType>,
    name: PlSmallStr,
}

impl<T> QuantityChuncked<T>
where
    T: PolarsNumericType,
    SeriesWrap<polars::prelude::ChunkedArray<T>>: polars::prelude::SeriesTrait
{
    fn from_series(&self, series: Series) -> PolarsResult<Self> {
        let quantity_array = series.struct_()?;
        is_valid_unit_dtype(quantity_array.dtype())?;
        let fields = &quantity_array.fields_as_series();
        let (value, unit) = (fields.get(0).unwrap(), fields.get(1).unwrap());
        Ok(Self {
            value: value.unpack::<T>().unwrap().clone(),
            unit: unit.str().unwrap().clone(),
            name: series.name().clone(),
        })
    }

    fn to_series(&self) -> PolarsResult<Series> {
        let fields = (self.value.into_series(), self.unit.into_series());
        Ok(
            StructChunked::from_series(self.name.clone(), self.value.len(), fields.iter())?
                .into_series(),
        )
    }
}

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

fn is_valid_unit_dtype(dtype: &DataType) -> PolarsResult<bool> {
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
    is_valid_unit_dtype(dtypes[0])?;
    Ok(Field::new("unit".into(), DataType::String))
}

#[polars_expr(output_type_func=unit_output)]
fn noop(inputs: &[Series]) -> PolarsResult<Series> {
    let struct_ = inputs[0].struct_()?;
    let fields = struct_.fields_as_series();

    if fields.is_empty() {
        return Ok(inputs[0].clone());
    }

    let fields = fields
        .iter()
        .map(|s| {
            let s = s.clone();
            println!("{:?}", s);
            s
        })
        .collect::<Vec<_>>();

    StructChunked::from_series(struct_.name().clone(), struct_.len(), fields.iter())
        .map(|ca| ca.into_series())
}

// fn apply_unary<F, T>(inputs: &[Series], func: F) -> PolarsResult<Series> where F: Fn(T) -> T, T: PolarsNumericType {
//     let quantity = inputs[0].struct_()?;
//     is_valid_unit_dtype(quantity.dtype())?;
//     let value = quantity.field_by_name("value")?;
//     let unit = quantity.field_by_name("unit")?;

// }
