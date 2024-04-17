# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..clients.client_gpt4 import GPT4
from ..functions.fx_classifymessage import BAMLClassifyMessage
from ..types.enums.enm_category import Category
from baml_core.jinja.render_prompt import RenderData
from baml_core.provider_manager.llm_response import LLMResponse
from baml_core.stream import AsyncStream
from baml_lib._impl.deserializer import Deserializer


import typing
# Impl: default_config
# Client: GPT4
# An implementation of ClassifyMessage.

__prompt_template = """\
Classify the following INPUT into ONE
of the following categories:

INPUT: {{ input }}

{{ ctx.output_schema }}

Response:\
"""

# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[Category](Category)  # type: ignore

# Add a deserializer that handles stream responses, which are all Partial types
__partial_deserializer = Deserializer[Category](Category)  # type: ignore

__output_schema = """
"Category as string"

Category
---
Refund
CancelOrder
TechnicalSupport
AccountIssue
Question
""".strip()

__template_macros = [
]


async def default_config(*, input: str) -> Category:
    response = await GPT4.run_jinja_template(
        jinja_template=__prompt_template,
        output_schema=__output_schema, template_macros=__template_macros,
        args=dict(input=input)
    )
    deserialized = __deserializer.from_string(response.generated)
    return deserialized


def default_config_stream(*, input: str
) -> AsyncStream[Category, Category]:
    def run_prompt() -> typing.AsyncIterator[LLMResponse]:
        raw_stream = GPT4.run_jinja_template_stream(
            jinja_template=__prompt_template,
            output_schema=__output_schema, template_macros=__template_macros,
            args=dict(input=input)
        )
        return raw_stream
    stream = AsyncStream(stream_cb=run_prompt, partial_deserializer=__partial_deserializer, final_deserializer=__deserializer)
    return stream

BAMLClassifyMessage.register_impl("default_config")(default_config, default_config_stream)