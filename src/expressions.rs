#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use crate::expressions::polars_plan::prelude::Expr;
use std::ops::Add;

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
    Ok(Field::new("unit".into(), DataType::Struct(
        vec![
            Field::new("value".into(), dtypes[0].clone()),
            Field::new("unit".into(), DataType::String),
        ]
    )))
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


fn extract_unit(input: &Series) -> PolarsResult<(Series, Series)>{
    let ca = input.struct_()?;
    check_valid_unit_dtype(ca.dtype())?;
    let fields = &ca.fields_as_series();
    let (value, unit) = (
        fields.get(0).unwrap().clone(),
        fields.get(1).unwrap().clone(),
    );
    check_same_unit(unit.str()?)?;

    Ok((value, unit))
}
fn apply_unary(input: &Series, expr: Expr) -> PolarsResult<Series> {
    let (value, unit) = extract_unit(input)?;
    let df = df!["value" => value]?.lazy().select(&[expr]).collect()?;
    let value = df["result"].clone().with_name("value".into());
    let fields = vec![value.as_series().unwrap().clone(), unit.into_series()];
    Ok(StructChunked::from_series(input.name().clone(), input.len(), fields.iter())?.into_series())
}

fn apply_binary(left: &Series, right: &Series, expr: Expr) -> PolarsResult<Series> {
    let (value_left, unit_left) = extract_unit(left)?;
    let (value_right, unit_right) = extract_unit(right)?;
    let df = df!["value_left" => value_left, "value_right" => value_right]?.lazy().select(&[expr]).collect()?;
    let result = df["result"].clone();

    if unit_left.first() != unit_right.first() {
        polars_bail!(InvalidOperation: "Expected units to be the same, got {:?} and {:?}", unit_left.first(), unit_right.first())
    }

    let fields = vec![result.as_series().unwrap().clone(), unit_left.into_series()];
    Ok(StructChunked::from_series(result.name().clone(), result.len(), fields.iter())?.into_series())
}


macro_rules! create_unit_unary_expr {
    ($name:ident) => {
        #[polars_expr(output_type_func=unit_output)]
        fn $name(inputs: &[Series]) -> PolarsResult<Series> {
            apply_unary(&inputs[0], col("value").$name().alias("result"))
        }
    };
}

macro_rules! create_unit_binary_expr {
    ($name:ident) => {
        #[polars_expr(output_type_func=unit_output)]
        fn $name(inputs: &[Series]) -> PolarsResult<Series> {
            apply_binary(&inputs[0], &inputs[1], col("value_left").$name(col("value_right")).alias("result"))
        }
    };
}


#[polars_expr(output_type_func=unit_output)]
fn noop(inputs: &[Series]) -> PolarsResult<Series> {
    apply_unary(&inputs[0], col("value").alias("result"))
}

// #[polars_expr(output_type_func=unit_output)]
// fn abs(inputs: &[Series]) -> PolarsResult<Series> {
//     apply_unary(&inputs[0], col("value").abs())
// }

create_unit_binary_expr!(add);
create_unit_unary_expr!(abs);
create_unit_unary_expr!(sin);
create_unit_unary_expr!(arccos);
create_unit_unary_expr!(arccosh);
create_unit_unary_expr!(arcsin);
create_unit_unary_expr!(arcsinh);
create_unit_unary_expr!(arctan);
create_unit_unary_expr!(arctanh);
create_unit_unary_expr!(arg_max);
create_unit_unary_expr!(arg_min);
// create_unit_expr!(arg_sort);
// create_unit_expr!(backward_fill);
// create_unit_expr!(cast);
create_unit_unary_expr!(cbrt);
// create_unit_expr!(ceil);
// create_unit_expr!(clip);
create_unit_unary_expr!(cos);
create_unit_unary_expr!(cosh);
create_unit_unary_expr!(cot);
