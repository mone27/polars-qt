#![allow(clippy::unused_unit)]

use std::ops::{Add, Div, Mul, Neg, Sub};

use polars::frame::column::ScalarColumn;
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

use crate::expressions::polars_plan::prelude::Expr;
use crate::units::*;

#[allow(clippy::get_first)]
fn check_valid_quantity_dtype(dtype: &DataType) -> PolarsResult<bool> {
    match dtype {
        DataType::Struct(fields) => {
            if let (Some(value_field), Some(unit_field)) = (fields.get(0), fields.get(1)) {
                Units::check_valid_unit_dtype(&unit_field.dtype)?;
                if value_field.name == "value"
                    && value_field.dtype.is_numeric()
                    && unit_field.name == "unit"
                {
                    Ok(true)
                } else {
                    polars_bail!(InvalidOperation: "Invalid Quantity. Expected struct with fields 'value' and 'unit' and types numeric and Unit, got {:?}", fields)
                }
            } else {
                polars_bail!(InvalidOperation: "Invalid Quantity. Expected struct with 2 fields ('value' and 'unit'), got {:?} fields ({:?})", fields.len(), fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>())
            }
        },
        dtype => polars_bail!(InvalidOperation: "Expected Struct dtype, got {}", dtype),
    }
}

fn is_all_same<T: PartialEq>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] == w[1])
}

fn quantity_output(input_fields: &[Field]) -> PolarsResult<Field> {
    let dtypes: Vec<&DataType> = input_fields.iter().map(|f| f.dtype()).collect();
    // simplying assumptions: the value dtype must be the same for all inputs
    if !is_all_same(&dtypes) {
        polars_bail!(InvalidOperation: "Expected all input fields to have the same dtype, got {:?}", dtypes)
    }
    // check that datatype is valid
    check_valid_quantity_dtype(dtypes[0])?;
    Ok(Field::new(
        "unit".into(),
        DataType::Struct(vec![
            Field::new("value".into(), dtypes[0].clone()),
            Field::new("unit".into(), DataType::String),
        ]),
    ))
}

fn check_same_unit(ca: &ListChunked) -> PolarsResult<()> {
    let mut iter = ca.iter();
    let first = iter.next().unwrap();
    if iter.all(|s| s == first) {
        Ok(())
    } else {
        polars_bail!(InvalidOperation: "Expected all units to be the same")
    }
}

#[allow(clippy::get_first)]
fn extract_quantity(input: &Series) -> PolarsResult<(Series, Series)> {
    let ca = input.struct_()?;
    check_valid_quantity_dtype(ca.dtype())?;
    let fields = &ca.fields_as_series();
    let (value, unit) = (
        fields.get(0).unwrap().clone(),
        fields.get(1).unwrap().clone(),
    );
    check_same_unit(unit.list()?)?;

    Ok((value, unit))
}

fn add_unit(series: Series, unit_val: Scalar) -> PolarsResult<Series> {
    let unit_col = ScalarColumn::new("unit".into(), unit_val, series.len());
    let (name, len) = (series.name().clone(), series.len());
    let fields = [series, unit_col.take_materialized_series()];
    Ok(StructChunked::from_series(name, len, fields.iter())?.into_series())
}

fn extract_result(df: DataFrame) -> Series {
    let idx = df.get_column_index("result").unwrap();
    df.take_columns()
        .remove(idx)
        .with_name("value".into())
        .take_materialized_series()
}

fn get_new_unit(
    unit_left: Scalar,
    unit_right: Scalar,
    unit_tfms: Option<fn(Units, Units) -> Units>,
) -> PolarsResult<Scalar> {
    Ok(if let Some(tfms) = unit_tfms {
        tfms(
            Units::from_scalar(unit_left)?,
            Units::from_scalar(unit_right)?,
        )
        .to_scalar()?
    } else if unit_left == unit_right {
        polars_bail!(InvalidOperation: "Expected units to be the same, got {:?} and {:?}", unit_left, unit_right)
    } else {
        unit_left
    })
}

fn apply_unary(input: &Series, expr: Expr) -> PolarsResult<Series> {
    let (value, unit) = extract_quantity(input)?;
    let df = df!["value" => value]?.lazy().select(&[expr]).collect()?;
    let result = extract_result(df);
    add_unit(result, unit.first())
}

fn apply_binary(
    left: &Series,
    right: &Series,
    expr: Expr,
    unit_tfms: Option<fn(Units, Units) -> Units>, // TODO: the absence of this should imply that the units should be the same, not sure this is a good API to use an Option for it
) -> PolarsResult<Series> {
    let (value_left, unit_left) = extract_quantity(left)?;
    let (value_right, unit_right) = extract_quantity(right)?;
    let new_unit = get_new_unit(unit_left.first(), unit_right.first(), unit_tfms)?;
    let df: DataFrame = df!["value_left" => value_left, "value_right" => value_right]?
        .lazy()
        .select(&[expr])
        .collect()?;
    let result = extract_result(df);
    add_unit(result, new_unit)
}

macro_rules! create_unit_unary_expr {
    ($name:ident $(, $arg:expr)*) => {
        #[polars_expr(output_type_func=quantity_output)]
        fn $name(inputs: &[Series]) -> PolarsResult<Series> {
            apply_unary(&inputs[0], col("value").$name($($arg),*).alias("result"))
        }
    };
}

macro_rules! create_unit_binary_expr {
    ($name:ident) => {
        #[polars_expr(output_type_func=quantity_output)]
        fn $name(inputs: &[Series]) -> PolarsResult<Series> {
            apply_binary(
                &inputs[0],
                &inputs[1],
                col("value_left").$name(col("value_right")).alias("result"),
                None,
            )
        }
    };
    ($name:ident, $tfms_unit:expr) => {
        #[polars_expr(output_type_func=quantity_output)]
        fn $name(inputs: &[Series]) -> PolarsResult<Series> {
            apply_binary(
                &inputs[0],
                &inputs[1],
                col("value_left").$name(col("value_right")).alias("result"),
                Some($tfms_unit),
            )
        }
    };
}

#[polars_expr(output_type_func=quantity_output)]
fn noop(inputs: &[Series]) -> PolarsResult<Series> {
    apply_unary(&inputs[0], col("value").alias("result"))
}

create_unit_unary_expr!(abs);
create_unit_binary_expr!(add);
create_unit_unary_expr!(arccos);
create_unit_unary_expr!(arccosh);
create_unit_unary_expr!(arcsin);
create_unit_unary_expr!(arcsinh);
create_unit_unary_expr!(arctan);
create_unit_unary_expr!(arctanh);
// create_unit_expr!(backward_fill);
// create_unit_expr!(cast);
create_unit_unary_expr!(cbrt);
// create_unit_expr!(ceil);
// create_unit_expr!(clip);
create_unit_unary_expr!(cos);
create_unit_unary_expr!(cosh);
create_unit_unary_expr!(cot);
// create_unit_unary_expr!(cum_count);
create_unit_unary_expr!(cum_max, false);
create_unit_unary_expr!(cum_min, false);
create_unit_unary_expr!(cum_prod, false);
// create_unit_unary_expr!(diff);

// create_unit_binary_expr!(dot);
create_unit_unary_expr!(neg);
// create_unit_unary_expr!(exp);
// create_unit_unary_expr!(expm1);
// create_unit_unary_expr!(floor);
// create_unit_unary_expr!(log);
// create_unit_unary_expr!(log1p);
// create_unit_unary_expr!(log10);
// create_unit_unary_expr!(log2);
// create_unit_unary_expr!(round);
// create_unit_unary_expr!(sign);
create_unit_unary_expr!(sqrt);
create_unit_unary_expr!(tan);
create_unit_unary_expr!(tanh);
create_unit_binary_expr!(sub);
create_unit_unary_expr!(sin);
create_unit_binary_expr!(mul, |a, b| a.multiply(&b));
create_unit_binary_expr!(div, |a, b| a.divide(&b));

create_unit_unary_expr!(min);
create_unit_unary_expr!(max);
create_unit_unary_expr!(mean);
create_unit_unary_expr!(median);
create_unit_unary_expr!(std, 1);
create_unit_unary_expr!(var, 1);
create_unit_unary_expr!(sum);
