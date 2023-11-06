# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.1.1-canary.6
# Generated Date: __DATE__
# Generated by: __USER__

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..types.classes.cls_conversation import Conversation
from ..types.classes.cls_message import Message
from ..types.classes.cls_proposedmessage import ProposedMessage
from ..types.enums.enm_messagesender import MessageSender
from typing import Protocol, runtime_checkable


import typing

import pytest

ImplName = type(None)

T = typing.TypeVar("T", bound=typing.Callable[..., typing.Any])
CLS = typing.TypeVar("CLS", bound=type)


ITextPolisherOutput = str

@runtime_checkable
class ITextPolisher(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: ProposedMessage

    Returns:
        str
    """

    async def __call__(self, arg: ProposedMessage, /) -> str:
        ...


class BAMLTextPolisherImpl:
    async def run(self, arg: ProposedMessage, /) -> str:
        ...

class IBAMLTextPolisher:
    def register_impl(
        self, name: ImplName
    ) -> typing.Callable[[ITextPolisher], ITextPolisher]:
        ...

    def get_impl(self, name: ImplName) -> BAMLTextPolisherImpl:
        ...

    @typing.overload
    def test(self, test_function: T) -> T:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the TextPolisherInterface.

        Args:
            test_function : T
                The test function to be decorated.

        Usage:
            ```python
            # All implementations will be tested.

            @baml.TextPolisher.test
            def test_logic(TextPolisherImpl: ITextPolisher) -> None:
                result = await TextPolisherImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, *, exclude_impl: typing.Iterable[ImplName]) -> pytest.MarkDecorator:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the TextPolisherInterface.

        Args:
            exclude_impl : Iterable[ImplName]
                The names of the implementations to exclude from testing.

        Usage:
            ```python
            # All implementations except "" will be tested.

            @baml.TextPolisher.test(exclude_impl=[""])
            def test_logic(TextPolisherImpl: ITextPolisher) -> None:
                result = await TextPolisherImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, test_class: typing.Type[CLS]) -> typing.Type[CLS]:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the TextPolisherInterface.

        Args:
            test_class : Type[CLS]
                The test class to be decorated.

        Usage:
        ```python
        # All implementations will be tested in every test method.

        @baml.TextPolisher.test
        class TestClass:
            def test_a(self, TextPolisherImpl: ITextPolisher) -> None:
                ...
            def test_b(self, TextPolisherImpl: ITextPolisher) -> None:
                ...
        ```
        """
        ...

BAMLTextPolisher: IBAMLTextPolisher
