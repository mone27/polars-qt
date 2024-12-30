from __future__ import annotations

__all__ = [
    "noop",
    "add",
    "abs",
    "sin",
    "arccos",
    "arccosh",
    "arcsin",
    "arcsinh",
    "arctan",
    "arctanh",
    "arg_max",
    "arg_min",
    "cbrt",
    "cos",
    "cosh",
    "cot",
    "cum_max",
    "cum_min",
    "cum_prod",
    "dot",
    "sqrt",
    "tan",
    "tanh",
    "sub",
    "mul",
    "div",
    "min",
    "max",
    "mean",
    "median",
    "std",
    "var",
    "sum",
    "pow",
]


from pathlib import Path
from typing import TYPE_CHECKING, Any

import polars as pl
from polars.plugins import register_plugin_function

from polars_qt._internal import __version__ as __version__

if TYPE_CHECKING:
    from polars_qt.typing import IntoExprColumn

LIB = Path(__file__).parent


def plugin_fn(name: str, *args, is_elem=True,  kwargs: dict[str, Any] | None = None) -> pl.Expr:
    return register_plugin_function(
        args=list(args),
        plugin_path=LIB,
        function_name=name,
        is_elementwise=is_elem,
        kwargs=kwargs,
    )


def noop(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("noop", expr)


def add(expr_a: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr:
    return plugin_fn("add", expr_a, expr_b)


def abs(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("abs", expr)


def sin(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("sin", expr)


def arccos(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arccos", expr)


def arccosh(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arccosh", expr)


def arcsin(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arcsin", expr)


def arcsinh(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arcsinh", expr)


def arctan(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arctan", expr)


def arctanh(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arctanh", expr)


def arg_max(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arg_max", expr)


def arg_min(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("arg_min", expr)


def cbrt(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("cbrt", expr)


def cos(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("cos", expr)


def cosh(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("cosh", expr)


def cot(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("cot", expr)


def cum_max(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("cum_max", expr, is_elem=False)


def cum_min(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("cum_min", expr, is_elem=False)


def cum_prod(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("cum_prod", expr, is_elem=False)


def dot(expr_a: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr:
    return plugin_fn("dot", expr_a, expr_b)


def neg(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("neg", expr)


def sqrt(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("sqrt", expr)


def tan(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("tan", expr)


def tanh(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("tanh", expr)


def sub(expr_a: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr:
    return plugin_fn("sub", expr_a, expr_b)


def mul(expr_a: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr:
    return plugin_fn("mul", expr_a, expr_b)


def div(expr_a: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr:
    return plugin_fn("div", expr_a, expr_b)


def min(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("min", expr, is_elem=False)


def max(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("max", expr, is_elem=False)


def mean(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("mean", expr, is_elem=False)


def median(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("median", expr, is_elem=False)


def std(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("std", expr)


def var(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("var", expr, is_elem=False)


def sum(expr: IntoExprColumn) -> pl.Expr:
    return plugin_fn("sum", expr, is_elem=False)


def pow(expr, exp: int | float) -> pl.Expr:
    if isinstance(exp, int):
        return plugin_fn("pow_int", expr, kwargs={"exp": exp})
    elif isinstance(exp, float):
        return plugin_fn("pow_float", expr, kwargs={"exp": exp})
    else:
        raise ValueError("Exponenet must be int or float for quantities")
    