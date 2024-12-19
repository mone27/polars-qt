#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;

// struct Quantity{
//     value: i64,
//     unit: String,
// }


// fn struct_quantity_output(input_fields: &[Field]) -> PolarsResult<Field> {
//     let field = &input_fields[0];
//     match field.dtype() {
//         DataType::Struct(fields) => {
//             Ok(Field::new("struct_quatity".into(), DataType::Struct(fields.clone())))
//         }
//         dtype => polars_bail!(InvalidOperation: "expected Struct dtype, got {}", dtype),
//     }
// }

// #[polars_expr(output_type=String)]
// fn unit_sum(inputs: &[Series]) -> PolarsResult<Series> {
//     // let a = inputs[0].struct_()?;
//     // let b = inputs[1].struct_()?;

//     // let a_fields = a.fields_as_series();
//     // let b_fields = b.fields_as_series();

//     // if (a_fields != b_fields) {
//     //     polars_bail!(InvalidOperation: "series must have some fields, got {} and {}", a_fields, b_fields)
//     // }

//     // check is the unit is the same

//     let out = Series::new("sum".to_string(), "Ok".to_string());
//     Ok(out.into_series())
// }



fn unit_output(input_fields: &[Field]) -> PolarsResult<Field> {
    let field = &input_fields[0];
    match field.dtype() {
        DataType::Struct(fields) => {
            if let (Some(value_field), Some(unit_field)) = (fields.get(0), fields.get(1)){
                Ok(Field::new("struct_point_2d".into(), DataType::Struct(fields.clone())))
            } else {
                polars_bail!(InvalidOperation: "wrong fields")
            }
        }
        dtype => polars_bail!(InvalidOperation: "expected Struct dtype, got {}", dtype),
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