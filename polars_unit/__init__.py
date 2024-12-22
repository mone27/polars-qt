from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Callable

import polars as pl
from polars.plugins import register_plugin_function

from polars_unit._internal import __version__ as __version__

if TYPE_CHECKING:
    from polars_unit.typing import IntoExprColumn

LIB = Path(__file__).parent

def plugin_function_unary(name: str) -> Callable:
    def decorator(func: Callable) -> Callable:
        def wrapper(expr: IntoExprColumn) -> pl.Expr:
            return register_plugin_function(
                args=list(expr),
                plugin_path=LIB,
                function_name=name,
                is_elementwise=True,
            )
        return wrapper
    return decorator

def plugin_function_binary(name: str) -> Callable:
    def decorator(func: Callable) -> Callable:
        def wrapper(expr_a: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr:
            return register_plugin_function(
                args=[expr_a, expr_b],
                plugin_path=LIB,
                function_name=name,
                is_elementwise=True,
            )
        return wrapper
    return decorator

# @plugin_function_unary("noop")
# def noop(expr: IntoExprColumn) -> pl.Expr: pass

def noop(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="noop",
        is_elementwise=True,
    )

def abs(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="abs",
        is_elementwise=True,
    )

def sin(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="sin",
        is_elementwise=True,
    )

# @plugin_function_unary("abs")
# def abs(expr: IntoExprColumn) -> pl.Expr: pass

# @plugin_function_unary("neg")
# def neg(expr: IntoExprColumn) -> pl.Expr: pass

# @plugin_function_binary("add")
# def add(expr_a: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr: pass

# @plugin_function_unary("sin")
# def sin(expr: IntoExprColumn, expr_b: IntoExprColumn) -> pl.Expr: pass
