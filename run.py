import polars as pl
from polars_unit import unit_sum

quantity = pl.Struct({
    'value': pl.Int64,
    'unit': pl.String,
})


df = pl.DataFrame({
    'a': pl.Series([
        (1, 'm'),
        (2, 'm'),
        (3, 'm'),
        (4, 'm'),
        (5, 'm'),
    ], dtype=quantity),
    
    'b': pl.Series([
        (1, 'm'),
        (2, 'm'),
        (3, 'm'),
        (4, 'm'),
        (5, 'm'),
    ], dtype=quantity),
    'c': [1,2,3, 4, 5]
})


result = df.with_columns(sum = unit_sum("a"), sum2 = unit_sum("c"))
print(result)
