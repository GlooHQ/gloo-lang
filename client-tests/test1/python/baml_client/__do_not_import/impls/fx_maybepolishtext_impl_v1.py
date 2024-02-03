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
from ..types.classes.cls_improvedresponse import ImprovedResponse
from ..types.classes.cls_message import Message
from ..types.classes.cls_proposedmessage import ProposedMessage
from ..types.enums.enm_messagesender import MessageSender
from ..types.enums.enm_sentiment import Sentiment
from baml_lib._impl.deserializer import Deserializer
from typing import Callable
from baml_lib._impl.functions import OnStreamCallable


# Impl: v1
# Client: AZURE_GPT4
# An implementation of .


__prompt_template = """\
Given a conversation with a resident, consider improving the response previously shown.

Good responses are amiable and direct.

Do not use or negative unless the question is a yes or no question.

```input
{arg}
```       


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
    "{arg}"
}


# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[ImprovedResponse](ImprovedResponse)  # type: ignore
__deserializer.overload("ImprovedResponse", {"ShouldImprove": "should_improve"})






async def v1(arg: ProposedMessage, /) -> ImprovedResponse:
    response = await AZURE_GPT4.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(arg=arg))
    deserialized = __deserializer.from_string(response.generated)
    return deserialized

from pydantic import BaseModel
import typing
from typing import final, Tuple, Union, Literal
from abc import ABC, abstractmethod
class StreamResponse(ABC):
    @abstractmethod
    def is_complete(self) -> bool:
        pass

@final
class StreamResponsePartial(StreamResponse):
    partial_response: Partial[ImprovedResponse] # but for str outputs it's just a str
    delta: str

    def __init__(self, partial_response: Partial[ImprovedResponse], delta: str):
        self.partial_response = partial_response
        self.delta = delta

    def is_complete(self) -> bool:
        return False

@final
class StreamResponseFinal(StreamResponse):
    response: ImprovedResponse
    def __init__(self, response: ImprovedResponse):
        self.response = response

    def is_complete(self) -> bool:
        return True


async def test_stream() -> typing.AsyncGenerator[Union[Tuple[Literal['partial'], StreamResponsePartial], Tuple[Literal['final'], StreamResponseFinal]], None]:
    """
    ```python
    async def caller_func() -> None:
        async for s in test_stream():
            match s[0]:
                case 'final':
                    response = s[1].response
                    print(s[1].response)
                case 'partial':
                    stream = s[1]
    ```
    """

    yield ('final', StreamResponseFinal(response=ImprovedResponse(should_improve=True, improved_response="Improved response", field=Sentiment.Negative)))
    yield ('partial', StreamResponsePartial(partial_response=ImprovedResponse(should_improve=True, improved_response="Improved response", field=Sentiment.Negative), delta="delta"))


async def test_stream_with_callback(callback: Callable[[Union[StreamResponsePartial, StreamResponseFinal]], None]) -> ImprovedResponse:
    async for s in test_stream():
        callback(s[1])

    return ImprovedResponse(should_improve=True, improved_response="Improved response", field=Sentiment.Negative)


class Unset:
    pass
_UNSET = Unset()
async def caller_func() -> None:
    final_response = None  # Use None as the initial unset state

    async for s in test_stream():
        if s[0] == 'final':
            final_response = s[1]  # Directly store the final response
            print(final_response)
        elif s[0] == 'partial':
            # Process partial responses if necessary
            print(s[1])  # Assuming s[1] is directly usable or represents the stream

    # After the loop, check if the final response was set
    if final_response is None:
        raise ValueError("Final response was not set.")
    
    actual_response = final_response.response

    
    # Assuming final_response has the attribute should_improve for further operations
    # Perform operations with final_response.should_improve

    # If there's a need to handle a stream with a callback
    await test_stream_with_callback(lambda x: print(x))


    response = await test_stream_with_callback(lambda x: print(x))

# how to create generator from the callback:
async def create_generator_from_callback(callback: Callable[[Union[StreamResponsePartial, StreamResponseFinal]], None]) -> typing.AsyncGenerator[Union[StreamResponsePartial, StreamResponseFinal], None]:
    async for s in test_stream():
        callback(s[1])
        yield s[1]

async def call_generator() -> None:
    response =  create_generator_from_callback(lambda x: print(x))

    for r in response:
        

# TODO: use async generator
# -> AsyncGenerator[StreamResponse]
async def v1_stream(arg: ProposedMessage, /, __onstream__: OnStreamCallable) -> ImprovedResponse:
    # Since the original operation was commented out, we'll implement a placeholder
    # This should mimic the streaming operation, calling __onstream__ with a string
    
    # Placeholder: simulate streaming by calling __onstream__ with a mock response
    mock_stream_response = "Streaming response part"  # Mock response part for demonstration
    # __onstream__(mock_stream_response)  # Call the __onstream__ callback with mock data
    
    # Here you would have your actual streaming logic, something like:
    response = await AZURE_GPT4.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(arg=arg))
    for part in response:
        __onstream__(part)

    # Placeholder for the final return, since the real implementation is commented out
    # In a real scenario, you would return the final or aggregated response from the stream
    return ImprovedResponse(should_improve=True, improved_response="Improved response", field=Sentiment.Negative) 

BAMLMaybePolishText.register_impl("v1")(v1, v1_stream)
