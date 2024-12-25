# Polars unit

polars plugin to support physical unit


This package is still in early phases of development. Supported features:

- [x] use expression transparently on the numeric columns
- [ ] functions that take an argument (e.g. `clip`) that is not an expression
- [ ] propagate physical units when combining different units

The value and physical unit is stored as a struct

## Example

add `unit.with_("m")` to a `Series` to specify the unit of measure.


```python
import polars as pl
import polars_unit as plu

df = pl.DataFrame({
    "distance": pl.Series([1.0, 2.0, 3.0]).unit.with_("m"),
    "time": pl.Series([1.0, 2.0, 3.0]).unit.with_("s"),
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
<small>shape: (3, 2)</small><table border="1" class="dataframe"><thead><tr><th>distance</th><th>time</th></tr><tr><td>struct[2]</td><td>struct[2]</td></tr></thead><tbody><tr><td>{1.0,&quot;m&quot;}</td><td>{1.0,&quot;s&quot;}</td></tr><tr><td>{2.0,&quot;m&quot;}</td><td>{2.0,&quot;s&quot;}</td></tr><tr><td>{3.0,&quot;m&quot;}</td><td>{3.0,&quot;s&quot;}</td></tr></tbody></table></div>



Can apply functions on the underlying numeric column using `.unit.<func>` on an expression


```python
df.select(pl.col("distance").unit.mean())
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (1, 1)</small><table border="1" class="dataframe"><thead><tr><th>distance</th></tr><tr><td>struct[2]</td></tr></thead><tbody><tr><td>{2.0,&quot;m&quot;}</td></tr></tbody></table></div>



`<func>` can be any expression function that is supported on a numeric column. It also works on functions that take 2 columns


```python
df.with_columns(
    dist_neg = pl.col("distance").unit.neg(),
    dist_dist = pl.col("distance").unit.add(pl.col("distance"))
)
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (3, 4)</small><table border="1" class="dataframe"><thead><tr><th>distance</th><th>time</th><th>dist_neg</th><th>dist_dist</th></tr><tr><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td></tr></thead><tbody><tr><td>{1.0,&quot;m&quot;}</td><td>{1.0,&quot;s&quot;}</td><td>{-1.0,&quot;m&quot;}</td><td>{2.0,&quot;m&quot;}</td></tr><tr><td>{2.0,&quot;m&quot;}</td><td>{2.0,&quot;s&quot;}</td><td>{-2.0,&quot;m&quot;}</td><td>{4.0,&quot;m&quot;}</td></tr><tr><td>{3.0,&quot;m&quot;}</td><td>{3.0,&quot;s&quot;}</td><td>{-3.0,&quot;m&quot;}</td><td>{6.0,&quot;m&quot;}</td></tr></tbody></table></div>



you need to use the `.unit` on at least one operand (cannot subclass `pl.Series`) when doing basic arithmetic


```python
df.with_columns(
    dist_squared = pl.col("distance").unit * pl.col("distance") 
)
```




<div><style>
.dataframe > thead > tr,
.dataframe > tbody > tr {
  text-align: right;
  white-space: pre-wrap;
}
</style>
<small>shape: (3, 5)</small><table border="1" class="dataframe"><thead><tr><th>distance</th><th>time</th><th>dist_neg</th><th>dist_dist</th><th>dist_squared</th></tr><tr><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td><td>struct[2]</td></tr></thead><tbody><tr><td>{1.0,&quot;m&quot;}</td><td>{1.0,&quot;s&quot;}</td><td>{-1.0,&quot;m&quot;}</td><td>{2.0,&quot;m&quot;}</td><td>{1.0,&quot;m&quot;}</td></tr><tr><td>{2.0,&quot;m&quot;}</td><td>{2.0,&quot;s&quot;}</td><td>{-2.0,&quot;m&quot;}</td><td>{4.0,&quot;m&quot;}</td><td>{4.0,&quot;m&quot;}</td></tr><tr><td>{3.0,&quot;m&quot;}</td><td>{3.0,&quot;s&quot;}</td><td>{-3.0,&quot;m&quot;}</td><td>{6.0,&quot;m&quot;}</td><td>{9.0,&quot;m&quot;}</td></tr></tbody></table></div>



it enforces that there can't be operations between different units (soon will calculate the appropriate unit)


```python
try:
    df.with_columns(
        speed = pl.col("distance").unit / pl.col("time")
    )
except Exception as e:
    print(e)
```

    the plugin failed with message: Expected units to be the same, got Scalar { dtype: String, value: StringOwned("m") } and Scalar { dtype: String, value: StringOwned("s") }


## Details

the plugin is implemented as a rust polars plugin. 

A *unit* `Series` is stored as a Struct with two fields:

- `value` a numeric column
- `unit` a string (soon to be Enum) with the unit

Polars doesn't support yet Extentions Dtype so this implementation detail is shown to the user.

The core of the plugin unpacks the `value` from the given series, applies the original expression, and then repacks it a *unit* Series


```python

```
