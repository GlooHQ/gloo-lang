# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..__do_not_import.generated_baml_client import baml
from ..baml_types import EnumOutput, IFnEnumOutput, IFnEnumOutputStream
from baml_lib._impl.deserializer import Deserializer
from json import dumps
from pytest_baml.ipc_channel import BaseIPCChannel
from typing import Any


@baml.FnEnumOutput.test(stream=False)
async def test_dependent_tomato(FnEnumOutputImpl: IFnEnumOutput, baml_ipc_channel: BaseIPCChannel):
    def to_str(item: Any) -> str:
        if isinstance(item, str):
            return item
        return dumps(item)

    content = to_str("noop")
    deserializer = Deserializer[str](str) # type: ignore
    param = deserializer.from_string(content)
    await FnEnumOutputImpl(param)

