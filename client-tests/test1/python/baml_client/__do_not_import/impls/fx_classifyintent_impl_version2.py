# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..clients.client_gpt4 import GPT4
from ..functions.fx_classifyintent import BAMLClassifyIntent
from ..types.enums.enm_intent import Intent
from baml_lib._impl.deserializer import Deserializer


# Impl: version2
# Client: GPT4
# An implementation of .


__prompt_template = """\
Classify the following INPUT into ONE
of the following Intents: 

Intent
---
k1: Customer wants to return a product
k2: Customer wants to cancel an order
technical-support: Customer needs help with a technical issue unrelated to account creation
account-issue: Specifically relates to account-creation

INPUT: {arg}

Response:\
"""

__input_replacers = {
    "{arg}"
}


# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[Intent](Intent)  # type: ignore
__deserializer.overload("Intent", {"technical-support": "TechnicalSupport", "account-issue": "AccountIssue"})






@BAMLClassifyIntent.register_impl("version2")
async def version2(arg: str, /) -> Intent:
    response = await GPT4.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(arg=arg))
    deserialized = __deserializer.from_string(response.generated)
    return deserialized
