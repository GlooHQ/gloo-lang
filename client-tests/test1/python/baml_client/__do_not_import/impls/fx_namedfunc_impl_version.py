# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..clients.client_azure_gpt4 import AZURE_GPT4
from ..functions.fx_namedfunc import BAMLNamedfunc
from ..types.classes.cls_basicclass import BasicClass
from ..types.partial.classes.cls_basicclass import PartialBasicClass
from baml_core.provider_manager.llm_response import LLMResponse
from baml_core.stream import AsyncStream
from baml_lib._impl.deserializer import Deserializer


import typing
# Impl: version
# Client: AZURE_GPT4
# An implementation of Namedfunc.

__prompt_template = """\
Given a userr is trying to schedule a meeting, extract the relevant information
{name}
information from the query.
JSON:\
"""

__input_replacers = {
    "{name}"
}


# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[str](str)  # type: ignore

# Add a deserializer that handles stream responses, which are all Partial types
__partial_deserializer = Deserializer[str](str)  # type: ignore







async def version(*, address: str, name: BasicClass) -> str:
    response = await AZURE_GPT4.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(address=address, name=name))
    deserialized = __deserializer.from_string(response.generated)
    return deserialized


def version_stream(*, address: str, name: BasicClass
) -> AsyncStream[str, str]:
    def run_prompt() -> typing.AsyncIterator[LLMResponse]:
        raw_stream = AZURE_GPT4.run_prompt_template_stream(template=__prompt_template, replacers=__input_replacers, params=dict(address=address, name=name))
        return raw_stream
    stream = AsyncStream(stream_cb=run_prompt, partial_deserializer=__partial_deserializer, final_deserializer=__deserializer)
    return stream

BAMLNamedfunc.register_impl("version")(version, version_stream)