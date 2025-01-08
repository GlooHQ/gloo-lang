import pytest
from assertpy import assert_that
from .test_setup import b
from ..baml_client.types import (
    LinkedList,
    Node,
    BinaryNode,
    Tree,
    Forest,
    LinkedListAliasNode,
    ClassToRecAlias,
    NodeWithAliasIndirection,
)

@pytest.mark.asyncio
async def test_simple_recursive_type():
    res = await b.BuildLinkedList([1, 2, 3, 4, 5])
    assert res == LinkedList(
        len=5,
        head=Node(
            data=1,
            next=Node(
                data=2,
                next=Node(data=3, next=Node(data=4, next=Node(data=5, next=None))),
            ),
        ),
    )


@pytest.mark.asyncio
async def test_mutually_recursive_type():
    res = await b.BuildTree(
        BinaryNode(
            data=5,
            left=BinaryNode(
                data=3,
                left=BinaryNode(
                    data=1, left=BinaryNode(data=2, left=None, right=None), right=None
                ),
                right=BinaryNode(data=4, left=None, right=None),
            ),
            right=BinaryNode(
                data=7,
                left=BinaryNode(data=6, left=None, right=None),
                right=BinaryNode(data=8, left=None, right=None),
            ),
        )
    )
    assert res == Tree(
        data=5,
        children=Forest(
            trees=[
                Tree(
                    data=3,
                    children=Forest(
                        trees=[
                            Tree(
                                data=1,
                                children=Forest(
                                    trees=[Tree(data=2, children=Forest(trees=[]))]
                                ),
                            ),
                            Tree(data=4, children=Forest(trees=[])),
                        ]
                    ),
                ),
                Tree(
                    data=7,
                    children=Forest(
                        trees=[
                            Tree(data=6, children=Forest(trees=[])),
                            Tree(data=8, children=Forest(trees=[])),
                        ]
                    ),
                ),
            ]
        ),
    )


@pytest.mark.asyncio
async def test_alias_pointing_to_recursive_class():
    res = await b.AliasThatPointsToRecursiveType(
        LinkedListAliasNode(value=1, next=None)
    )
    assert res == LinkedListAliasNode(value=1, next=None)


@pytest.mark.asyncio
async def test_class_pointing_to_alias_that_points_to_recursive_class():
    res = await b.ClassThatPointsToRecursiveClassThroughAlias(
        ClassToRecAlias(list=LinkedListAliasNode(value=1, next=None))
    )
    assert res == ClassToRecAlias(list=LinkedListAliasNode(value=1, next=None))


@pytest.mark.asyncio
async def test_recursive_class_with_alias_indirection():
    res = await b.RecursiveClassWithAliasIndirection(
        NodeWithAliasIndirection(
            value=1, next=NodeWithAliasIndirection(value=2, next=None)
        )
    )
    assert res == NodeWithAliasIndirection(
        value=1, next=NodeWithAliasIndirection(value=2, next=None)
    )