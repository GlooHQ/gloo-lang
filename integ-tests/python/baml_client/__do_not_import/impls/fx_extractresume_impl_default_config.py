# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

<<<<<<< HEAD
from ..clients.client_resilient_complexsyntax import Resilient_ComplexSyntax
||||||| parent of 209e3d97 (add examples)
from ..clients.client_ollama import Ollama
=======
from ..clients.client_gpt4 import GPT4
>>>>>>> 209e3d97 (add examples)
from ..functions.fx_extractresume import BAMLExtractResume
from ..types.classes.cls_resume import Resume
from ..types.partial.classes.cls_resume import PartialResume
from baml_core.jinja.render_prompt import RenderData
from baml_core.provider_manager.llm_response import LLMResponse
from baml_core.stream import AsyncStream
from baml_lib._impl.deserializer import Deserializer


import typing
# Impl: default_config
<<<<<<< HEAD
# Client: Resilient_ComplexSyntax
||||||| parent of 209e3d97 (add examples)
# Client: Ollama
=======
# Client: GPT4
>>>>>>> 209e3d97 (add examples)
# An implementation of ExtractResume.

__prompt_template = """\
Extract the following information from the resume:

Resume:
<<<<
{{ resume }}
<<<<

Output JSON schema:
{{ ctx.output_schema }}

JSON:\
"""

# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[Resume](Resume)  # type: ignore

# Add a deserializer that handles stream responses, which are all Partial types
__partial_deserializer = Deserializer[PartialResume](PartialResume)  # type: ignore

__output_schema = """
{
  "name": string,
  "email": string,
  "phone": string,
  "experience": string[],
  "education": string[],
  "skills": string[]
}
""".strip()

__template_macros = [
]


async def default_config(*, resume: str) -> Resume:
<<<<<<< HEAD
    response = await Resilient_ComplexSyntax.run_jinja_template(
||||||| parent of 209e3d97 (add examples)
    response = await Ollama.run_jinja_template(
=======
    response = await GPT4.run_jinja_template(
>>>>>>> 209e3d97 (add examples)
        jinja_template=__prompt_template,
        output_schema=__output_schema, template_macros=__template_macros,
        args=dict(resume=resume)
    )
    deserialized = __deserializer.from_string(response.generated)
    return deserialized


def default_config_stream(*, resume: str
) -> AsyncStream[Resume, PartialResume]:
    def run_prompt() -> typing.AsyncIterator[LLMResponse]:
<<<<<<< HEAD
        raw_stream = Resilient_ComplexSyntax.run_jinja_template_stream(
||||||| parent of 209e3d97 (add examples)
        raw_stream = Ollama.run_jinja_template_stream(
=======
        raw_stream = GPT4.run_jinja_template_stream(
>>>>>>> 209e3d97 (add examples)
            jinja_template=__prompt_template,
            output_schema=__output_schema, template_macros=__template_macros,
            args=dict(resume=resume)
        )
        return raw_stream
    stream = AsyncStream(stream_cb=run_prompt, partial_deserializer=__partial_deserializer, final_deserializer=__deserializer)
    return stream

BAMLExtractResume.register_impl("default_config")(default_config, default_config_stream)