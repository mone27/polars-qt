#![allow(clippy::unused_unit)]
use std::fmt::Write;

use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

fn unit_output(input_fields: &[Field]) -> PolarsResult<Field> {
    let field = &input_fields[0];
    match field.dtype() {
        DataType::Struct(fields) => {
            if let (Some(value_field), Some(unit_field)) = (fields.get(0), fields.get(1)) {
                if value_field.name == "value"
                    && value_field.dtype.is_numeric()
                    && unit_field.name == "unit"
                    && unit_field.dtype == DataType::String
                {
                    Ok(Field::new(
                        "struct_point_2d".into(),
                        DataType::Struct(fields.clone()),
                    ))
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
