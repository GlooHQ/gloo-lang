# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..types.classes.cls_conversation import Conversation
from ..types.classes.cls_message import Message
from ..types.enums.enm_messagesender import MessageSender
from typing import Optional, Protocol, runtime_checkable


import typing

import pytest

ImplName = typing.Literal["v1"]

T = typing.TypeVar("T", bound=typing.Callable[..., typing.Any])
CLS = typing.TypeVar("CLS", bound=type)


IMessageSimplifierOutput = Optional[int]

@runtime_checkable
class IMessageSimplifier(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: Conversation

    Returns:
        Optional[int]
    """

    async def __call__(self, arg: Conversation, /) -> Optional[int]:
        ...


class BAMLMessageSimplifierImpl:
    async def run(self, arg: Conversation, /) -> Optional[int]:
        ...

class IBAMLMessageSimplifier:
    def register_impl(
        self, name: ImplName
    ) -> typing.Callable[[IMessageSimplifier], IMessageSimplifier]:
        ...

    def get_impl(self, name: ImplName) -> BAMLMessageSimplifierImpl:
        ...

    @typing.overload
    def test(self, test_function: T) -> T:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the MessageSimplifierInterface.

        Args:
            test_function : T
                The test function to be decorated.

        Usage:
            ```python
            # All implementations will be tested.

            @baml.MessageSimplifier.test
            def test_logic(MessageSimplifierImpl: IMessageSimplifier) -> None:
                result = await MessageSimplifierImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, *, exclude_impl: typing.Iterable[ImplName]) -> pytest.MarkDecorator:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the MessageSimplifierInterface.

        Args:
            exclude_impl : Iterable[ImplName]
                The names of the implementations to exclude from testing.

        Usage:
            ```python
            # All implementations except "v1" will be tested.

            @baml.MessageSimplifier.test(exclude_impl=["v1"])
            def test_logic(MessageSimplifierImpl: IMessageSimplifier) -> None:
                result = await MessageSimplifierImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, test_class: typing.Type[CLS]) -> typing.Type[CLS]:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the MessageSimplifierInterface.

        Args:
            test_class : Type[CLS]
                The test class to be decorated.

        Usage:
        ```python
        # All implementations will be tested in every test method.

        @baml.MessageSimplifier.test
        class TestClass:
            def test_a(self, MessageSimplifierImpl: IMessageSimplifier) -> None:
                ...
            def test_b(self, MessageSimplifierImpl: IMessageSimplifier) -> None:
                ...
        ```
        """
        ...

BAMLMessageSimplifier: IBAMLMessageSimplifier
