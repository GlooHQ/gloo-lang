# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..clients.client_gpt4turbo import GPT4Turbo
from ..functions.fx_chainofthought import BAMLChainOfThought
from ..types.classes.cls_linteroutput import LinterOutput
from ..types.partial.classes.cls_linteroutput import PartialLinterOutput
from baml_core.provider_manager.llm_response import LLMResponse
from baml_core.stream import AsyncStream
from baml_lib._impl.deserializer import Deserializer
from typing import List


import typing
# Impl: version1
# Client: GPT4Turbo
# An implementation of ChainOfThought.

__prompt_template = """\
You are a powerful AI linter that only looks at one linting rule called "reasoning-rule".

The rule is this:
Given these INSTRUCTIONS below, return a diagnostic if they do not mention reasoning or thinking before outputting the answer. Try to focus on the part of the instructions that mentions the output schema if there are any of those directions

The rest of the text is not applicable to this linting rule. Only apply the linting rule to INSTRUCTIONS. 

--------------------
<INSTRUCTIONS>
{arg}
</INSTRUCTIONS>
--------------------


Output the diagnostic in this JSON format (only include these fields, and no others):
{
  // Explain why the linting error was raised.
  "reason": string,
  // The phrase that triggered the linter error. Write it EXACTLY as it appears in the PROMPT. If it's more than 10 words, just match the first 10 words.
  "exactPhrase": string,
  // A human-readable string that explains how to fix the linting error.
  "recommendation": string | null,
  // Explain why the recommendation is the best course of action.
  "recommendation_reason": string | null,
  // The fix for the linting error. You MUST start at the same location as the original phrase.
  "fixedPhrase": string | null
}[]

Output JSON Array:\
"""

__input_replacers = {
    "{arg}"
}


# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[List[LinterOutput]](List[LinterOutput])  # type: ignore

# Add a deserializer that handles stream responses, which are all Partial types
__partial_deserializer = Deserializer[List[LinterOutput]](List[LinterOutput])  # type: ignore







async def version1(arg: str, /) -> List[LinterOutput]:
    response = await GPT4Turbo.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(arg=arg))
    deserialized = __deserializer.from_string(response.generated)
    return deserialized


def version1_stream(arg: str, /) -> AsyncStream[List[LinterOutput], List[LinterOutput]]:
    def run_prompt() -> typing.AsyncIterator[LLMResponse]:
        raw_stream = GPT4Turbo.run_prompt_template_stream(template=__prompt_template, replacers=__input_replacers, params=dict(arg=arg))
        return raw_stream
    stream = AsyncStream(stream_cb=run_prompt, partial_deserializer=__partial_deserializer, final_deserializer=__deserializer)
    return stream

BAMLChainOfThought.register_impl("version1")(version1, version1_stream)