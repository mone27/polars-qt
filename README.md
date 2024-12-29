# Polars unit

polars plugin to support quantities: value and a physical unit


This package is still in early phases of development. Supported features:

- [x] use expression transparently on the numeric columns
- [ ] functions that take an argument (e.g. `clip`) that is not an expression
- [x] propagate physical units when combining different units
- [ ] unit conversions
- [ ] pretty print units

The value and physical unit is stored as a struct.

Unit operations supported:

- [x] addition/subtraction
- [x] multiplication/division
- [ ] power

## Example

add `qt.with_unit([("m", 1)])` to a `Series` to specify the unit of measure.


```python
import polars as pl
import polars_qt as plqt

df = pl.DataFrame({
    "distance": pl.Series([1.0, 2.0, 3.0]).qt.with_unit([("m", 1)]),
    "time": pl.Series([1.0, 2.0, 3.0]).qt.with_unit([("s", 1)]),
})
df
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (3, 2)</small><table border="1" class="dataframe"><thead><tr><th>distance</th><th>time</th></tr><tr><td>struct[2]</td><td>struct[2]</td></tr></thead><tbody><tr><td>{1.0,[{&quot;m&quot;,1}]}</td><td>{1.0,[{&quot;s&quot;,1}]}</td></tr><tr><td>{2.0,[{&quot;m&quot;,1}]}</td><td>{2.0,[{&quot;s&quot;,1}]}</td></tr><tr><td>{3.0,[{&quot;m&quot;,1}]}</td><td>{3.0,[{&quot;s&quot;,1}]}</td></tr></tbody></table></div>



Can apply functions on the underlying numeric column using `.qt.<func>` on an expression


```python
df.select(pl.col("distance").qt.mean())
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (1, 1)</small><table border="1" class="dataframe"><thead><tr><th>distance</th></tr><tr><td>struct[2]</td></tr></thead><tbody><tr><td>{2.0,[{&quot;m&quot;,1}]}</td></tr></tbody></table></div>



`<func>` can be any expression function that is supported on a numeric column. It also works on functions that take 2 columns


```python
df.with_columns(
    dist_neg = pl.col("distance").qt.neg(),
    dist_dist = pl.col("distance").qt.add(pl.col("distance"))
)
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (3, 4)</small><table border="1" class="dataframe"><thead><tr><th>distance</th><th>time</th><th>dist_neg</th><th>dist_dist</th></tr><tr><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td></tr></thead><tbody><tr><td>{1.0,[{&quot;m&quot;,1}]}</td><td>{1.0,[{&quot;s&quot;,1}]}</td><td>{-1.0,[{&quot;m&quot;,1}]}</td><td>{2.0,[{&quot;m&quot;,1}]}</td></tr><tr><td>{2.0,[{&quot;m&quot;,1}]}</td><td>{2.0,[{&quot;s&quot;,1}]}</td><td>{-2.0,[{&quot;m&quot;,1}]}</td><td>{4.0,[{&quot;m&quot;,1}]}</td></tr><tr><td>{3.0,[{&quot;m&quot;,1}]}</td><td>{3.0,[{&quot;s&quot;,1}]}</td><td>{-3.0,[{&quot;m&quot;,1}]}</td><td>{6.0,[{&quot;m&quot;,1}]}</td></tr></tbody></table></div>



you need to use the `.qt` on at least one operand (cannot subclass `pl.Series`) when doing basic arithmetic


```python
df.with_columns(
    dist_squared = pl.col("distance").qt * pl.col("distance") 
)
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (3, 3)</small><table border="1" class="dataframe"><thead><tr><th>distance</th><th>time</th><th>dist_squared</th></tr><tr><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td></tr></thead><tbody><tr><td>{1.0,[{&quot;m&quot;,1}]}</td><td>{1.0,[{&quot;s&quot;,1}]}</td><td>{1.0,[{&quot;m&quot;,2}]}</td></tr><tr><td>{2.0,[{&quot;m&quot;,1}]}</td><td>{2.0,[{&quot;s&quot;,1}]}</td><td>{4.0,[{&quot;m&quot;,2}]}</td></tr><tr><td>{3.0,[{&quot;m&quot;,1}]}</td><td>{3.0,[{&quot;s&quot;,1}]}</td><td>{9.0,[{&quot;m&quot;,2}]}</td></tr></tbody></table></div>




```python
df.with_columns(
    speed = pl.col("distance").qt / pl.col("time")
)
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (3, 3)</small><table border="1" class="dataframe"><thead><tr><th>distance</th><th>time</th><th>speed</th></tr><tr><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td></tr></thead><tbody><tr><td>{1.0,[{&quot;m&quot;,1}]}</td><td>{1.0,[{&quot;s&quot;,1}]}</td><td>{1.0,[{&quot;m&quot;,1}, {&quot;s&quot;,-1}]}</td></tr><tr><td>{2.0,[{&quot;m&quot;,1}]}</td><td>{2.0,[{&quot;s&quot;,1}]}</td><td>{1.0,[{&quot;m&quot;,1}, {&quot;s&quot;,-1}]}</td></tr><tr><td>{3.0,[{&quot;m&quot;,1}]}</td><td>{3.0,[{&quot;s&quot;,1}]}</td><td>{1.0,[{&quot;m&quot;,1}, {&quot;s&quot;,-1}]}</td></tr></tbody></table></div>



## Details

the plugin is implemented as a rust polars plugin. 

A *quantity* `Series` is stored as a Struct with two fields:

- `value` a numeric column
- `unit` a unit. This is implemented a a List column of Struct with the fields `name` and `power`

Polars doesn't support yet Extentions Dtype so this implementation detail is shown to the user.

The core of the plugin unpacks the `value` from the given series, applies the original expression, and then repacks it a *quantity* Series

### Unit system

we need a runtime unit system so we can't use the `uom` crate, which is a compile time unit check.

The idea of the implementation is that on the Rust side we only implement the basic checks of units and operations, all the rest (parsing from strings, simplify, formatting) will be implemented in python, maybe using a library like `pint` as a backend
