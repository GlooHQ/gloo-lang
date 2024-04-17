# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..types.enums.enm_category import Category
from baml_core.stream import AsyncStream
from typing import Callable, Protocol, runtime_checkable


import typing

import pytest
from contextlib import contextmanager
from unittest import mock

ImplName = typing.Literal["default_config"]

T = typing.TypeVar("T", bound=typing.Callable[..., typing.Any])
CLS = typing.TypeVar("CLS", bound=type)


IClassifyMessage3Output = Category

@runtime_checkable
class IClassifyMessage3(Protocol):
    """
    This is the interface for a function.

    Args:
        input: str

    Returns:
        Category
    """

    async def __call__(self, *, input: str) -> Category:
        ...

   

@runtime_checkable
class IClassifyMessage3Stream(Protocol):
    """
    This is the interface for a stream function.

    Args:
        input: str

    Returns:
        AsyncStream[Category, Category]
    """

    def __call__(self, *, input: str
) -> AsyncStream[Category, Category]:
        ...
class BAMLClassifyMessage3Impl:
    async def run(self, *, input: str) -> Category:
        ...
    
    def stream(self, *, input: str
) -> AsyncStream[Category, Category]:
        ...

class IBAMLClassifyMessage3:
    def register_impl(
        self, name: ImplName
    ) -> typing.Callable[[IClassifyMessage3, IClassifyMessage3Stream], None]:
        ...

    async def __call__(self, *, input: str) -> Category:
        ...

    def stream(self, *, input: str
) -> AsyncStream[Category, Category]:
        ...

    def get_impl(self, name: ImplName) -> BAMLClassifyMessage3Impl:
        ...

    @contextmanager
    def mock(self) -> typing.Generator[mock.AsyncMock, None, None]:
        """
        Utility for mocking the ClassifyMessage3Interface.

        Usage:
            ```python
            # All implementations are mocked.

            async def test_logic() -> None:
                with baml.ClassifyMessage3.mock() as mocked:
                    mocked.return_value = ...
                    result = await ClassifyMessage3Impl(...)
                    assert mocked.called
            ```
        """
        ...

    @typing.overload
    def test(self, test_function: T) -> T:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the ClassifyMessage3Interface.

        Args:
            test_function : T
                The test function to be decorated.

        Usage:
            ```python
            # All implementations will be tested.

            @baml.ClassifyMessage3.test
            async def test_logic(ClassifyMessage3Impl: IClassifyMessage3) -> None:
                result = await ClassifyMessage3Impl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, *, exclude_impl: typing.Iterable[ImplName] = [], stream: bool = False) -> pytest.MarkDecorator:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the ClassifyMessage3Interface.

        Args:
            exclude_impl : Iterable[ImplName]
                The names of the implementations to exclude from testing.
            stream: bool
                If set, will return a streamable version of the test function.

        Usage:
            ```python
            # All implementations except the given impl will be tested.

            @baml.ClassifyMessage3.test(exclude_impl=["implname"])
            async def test_logic(ClassifyMessage3Impl: IClassifyMessage3) -> None:
                result = await ClassifyMessage3Impl(...)
            ```

            ```python
            # Streamable version of the test function.

            @baml.ClassifyMessage3.test(stream=True)
            async def test_logic(ClassifyMessage3Impl: IClassifyMessage3Stream) -> None:
                async for result in ClassifyMessage3Impl(...):
                    ...
            ```
        """
        ...

    @typing.overload
    def test(self, test_class: typing.Type[CLS]) -> typing.Type[CLS]:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the ClassifyMessage3Interface.

        Args:
            test_class : Type[CLS]
                The test class to be decorated.

        Usage:
        ```python
        # All implementations will be tested in every test method.

        @baml.ClassifyMessage3.test
        class TestClass:
            def test_a(self, ClassifyMessage3Impl: IClassifyMessage3) -> None:
                ...
            def test_b(self, ClassifyMessage3Impl: IClassifyMessage3) -> None:
                ...
        ```
        """
        ...

BAMLClassifyMessage3: IBAMLClassifyMessage3
