# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.1.1-canary.12
# Generated Date: __DATE__
# Generated by: __USER__

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from baml_lib._impl.functions import BaseBAMLFunction
from typing import Protocol, runtime_checkable


IBlahOutput = str

@runtime_checkable
class IBlah(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: str

    Returns:
        str
    """

    async def __call__(self, arg: str, /) -> str:
        ...


class IBAMLBlah(BaseBAMLFunction[str]):
    def __init__(self) -> None:
        super().__init__(
            "Blah",
            IBlah,
            ["v1"],
        )

BAMLBlah = IBAMLBlah()

__all__ = [ "BAMLBlah" ]
