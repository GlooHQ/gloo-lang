# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..__do_not_import.generated_baml_client import baml
from ..baml_types import ClassifyResponse, IBClassifyTool, IBClassifyToolStream, Tool
from baml_lib._impl.deserializer import Deserializer
from json import dumps
from pytest_baml.ipc_channel import BaseIPCChannel
from typing import Any


@baml.BClassifyTool.test(stream=True)
async def test_back_lavender(BClassifyToolImpl: IBClassifyToolStream, baml_ipc_channel: BaseIPCChannel):
    def to_str(item: Any) -> str:
        if isinstance(item, str):
            return item
        return dumps(item)

    case = {"query": "lorem", "context": "ipsum", }
    deserializer_query = Deserializer[str](str) # type: ignore
    query = deserializer_query.from_string(to_str(case["query"]))
    deserializer_context = Deserializer[str](str) # type: ignore
    context = deserializer_context.from_string(to_str(case["context"]))
    async with BClassifyToolImpl(
        query=query,
        context=context
    ) as stream:
        async for response in stream.parsed_stream:
            baml_ipc_channel.send("partial_response", response.json())

        await stream.get_final_response()
