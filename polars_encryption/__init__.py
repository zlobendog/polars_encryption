from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import polars as pl

from polars_encryption.utils import parse_into_expr, register_plugin, parse_version

if TYPE_CHECKING:
    from polars.type_aliases import IntoExpr

if parse_version(pl.__version__) < parse_version("0.20.16"):
    from polars.utils.udfs import _get_shared_lib_location

    lib: str | Path = _get_shared_lib_location(__file__)
else:
    lib = Path(__file__).parent

def encrypt(expr: IntoExpr, *, key: bytes, nonce: bytes) -> pl.Expr:
    expr = parse_into_expr(expr)
    key_list = list(key)
    nonce_list = list(nonce)
    return register_plugin(
        args=[expr],
        symbol="encrypt",
        is_elementwise=True,
        lib=lib,
        kwargs={"key": key_list, "nonce": nonce_list},
    )

def decrypt(expr: IntoExpr, *, key: bytes, nonce: bytes) -> pl.Expr:
    expr = parse_into_expr(expr)
    key_list = list(key)
    nonce_list = list(nonce)
    return register_plugin(
        args=[expr],
        symbol="decrypt",
        is_elementwise=True,
        lib=lib,
        kwargs={"key": key_list, "nonce": nonce_list},
    )