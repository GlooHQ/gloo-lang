# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..clients.client_azure_gpt4 import AZURE_GPT4
from ..functions.fx_maybepolishtext import BAMLMaybePolishText
from ..types.classes.cls_conversation import Conversation
from ..types.classes.cls_dummyobj import DummyObj
from ..types.classes.cls_improvedresponse import ImprovedResponse
from ..types.classes.cls_message import Message
from ..types.classes.cls_proposedmessage import ProposedMessage
from ..types.enums.enm_messagesender import MessageSender
from ..types.enums.enm_sentiment import Sentiment
from baml_lib._impl.deserializer import Deserializer


# Impl: v1
# Client: AZURE_GPT4
# An implementation of .


__prompt_template = """\
Given a conversation with a resident, consider improving the response previously shown.

Good responses are amiable and direct.

Do not use or negative unless the question is a yes or no question.



Sentiment
---
Positive
Negative
Neutral


Output JSON Format:
{
  // false if the response is already contextual and pleasant
  "ShouldImprove": bool,
  // string if should_improve else null
  "improved_response": string | null,
  "field": "Sentiment as string"
}

JSON:\
"""

__input_replacers = {
}


# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[ImprovedResponse](ImprovedResponse)  # type: ignore
__deserializer.overload("ImprovedResponse", {"ShouldImprove": "should_improve"})






@BAMLMaybePolishText.register_impl("v1")
async def v1(arg: ProposedMessage, /) -> ImprovedResponse:
    response = await AZURE_GPT4.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(arg=arg))
    deserialized = __deserializer.from_string(response.generated)
    return deserialized
