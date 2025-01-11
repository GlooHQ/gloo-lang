import pytest
import asyncio
from assertpy import assert_that
from baml_py.errors import BamlValidationError
from .test_setup import b, sync_b
from ..baml_client import partial_types

@pytest.mark.asyncio
async def test_streaming():
    stream = b.stream.PromptTestStreaming(
        input="Programming languages are fun to create"
    )
    msgs = []

    start_time = asyncio.get_event_loop().time()
    last_msg_time = start_time
    first_msg_time = start_time + 10
    async for msg in stream:
        msgs.append(str(msg))
        if len(msgs) == 1:
            first_msg_time = asyncio.get_event_loop().time()

        last_msg_time = asyncio.get_event_loop().time()

    final = await stream.get_final_response()

    assert (
        first_msg_time - start_time <= 1.5
    ), "Expected first message within 1 second but it took longer."
    assert (
        last_msg_time - start_time >= 1
    ), "Expected last message after 1.5 seconds but it was earlier."
    assert len(final) > 0, "Expected non-empty final but got empty."
    assert len(msgs) > 0, "Expected at least one streamed response but got none."
    for prev_msg, msg in zip(msgs, msgs[1:]):
        assert msg.startswith(prev_msg), (
            "Expected messages to be continuous, but prev was %r and next was %r"
            % (prev_msg, msg,)
        )
    assert msgs[-1] == final, "Expected last stream message to match final response."


@pytest.mark.asyncio
async def test_streaming_uniterated():
    final = await b.stream.PromptTestStreaming(
        input="The color blue makes me sad"
    ).get_final_response()
    assert len(final) > 0, "Expected non-empty final but got empty."


def test_streaming_sync():
    stream = sync_b.stream.PromptTestStreaming(
        input="Programming languages are fun to create"
    )
    msgs = []

    start_time = asyncio.get_event_loop().time()
    last_msg_time = start_time
    first_msg_time = start_time + 10
    for msg in stream:
        msgs.append(str(msg))
        if len(msgs) == 1:
            first_msg_time = asyncio.get_event_loop().time()

        last_msg_time = asyncio.get_event_loop().time()

    final = stream.get_final_response()

    assert (
        first_msg_time - start_time <= 1.5
    ), "Expected first message within 1 second but it took longer."
    assert (
        last_msg_time - start_time >= 1
    ), "Expected last message after 1.5 seconds but it was earlier."
    assert len(final) > 0, "Expected non-empty final but got empty."
    assert len(msgs) > 0, "Expected at least one streamed response but got none."
    for prev_msg, msg in zip(msgs, msgs[1:]):
        assert msg.startswith(prev_msg), (
            "Expected messages to be continuous, but prev was %r and next was %r"
            % (prev_msg, msg,)
        )
    assert msgs[-1] == final, "Expected last stream message to match final response."


def test_streaming_uniterated_sync():
    final = sync_b.stream.PromptTestStreaming(
        input="The color blue makes me sad"
    ).get_final_response()
    assert len(final) > 0, "Expected non-empty final but got empty."


@pytest.mark.asyncio
async def test_nested_class_streaming():
    stream = b.stream.FnOutputClassNested(
        input="My name is Harrison. My hair is black and I'm 6 feet tall."
    )
    msgs = []
    async for msg in stream:
        print("streamed ", msg.model_dump(mode="json"))
        msgs.append(msg)
    final = await stream.get_final_response()

    assert len(msgs) > 0, "Expected at least one streamed response but got none."
    print("final ", final.model_dump(mode="json"))


@pytest.mark.asyncio
async def test_no_stream_big_integer():
    stream = b.stream.StreamOneBigNumber(digits=12)
    msgs = []
    async for msg in stream:
        msgs.append(msg)
    res = await stream.get_final_response()
    for msg in msgs:
        assert True if msg is None else msg == res


@pytest.mark.asyncio
async def test_no_stream_object_with_numbers():
    stream = b.stream.StreamBigNumbers(digits=12)
    msgs = []
    async for msg in stream:
        msgs.append(msg)
    res = await stream.get_final_response()

    # If Numbers aren't being streamed, then for every message, the partial
    # field should either be None, or exactly the value in the final result.
    for msg in msgs:
        assert True if msg.a is None else msg.a == res.a
        assert True if msg.b is None else msg.b == res.b


@pytest.mark.asyncio
async def test_no_stream_compound_object():
    stream = b.stream.StreamingCompoundNumbers(digits=12, yapping=False)
    msgs = []
    async for msg in stream:
        msgs.append(msg)
    res = await stream.get_final_response()
    for msg in msgs:
        if msg.big is not None:
            assert True if msg.big.a is None else msg.big.a == res.big.a
            assert True if msg.big.b is None else msg.big.b == res.big.b
        for msgEntry, resEntry in zip(msg.big_nums, res.big_nums):
            assert True if msgEntry.a is None else msgEntry.a == resEntry.a
            assert True if msgEntry.b is None else msgEntry.b == resEntry.b
        if msg.another is not None:
            assert True if msg.another.a is None else msg.another.a == res.another.a
            assert True if msg.another.b is None else msg.another.b == res.another.b


@pytest.mark.asyncio
async def test_no_stream_compound_object_with_yapping():
    stream = b.stream.StreamingCompoundNumbers(digits=12, yapping=True)
    msgs = []
    async for msg in stream:
        msgs.append(msg)
    res = await stream.get_final_response()
    for msg in msgs:
        if msg.big is not None:
            assert True if msg.big.a is None else msg.big.a == res.big.a
            assert True if msg.big.b is None else msg.big.b == res.big.b
        for msgEntry, resEntry in zip(msg.big_nums, res.big_nums):
            assert True if msgEntry.a is None else msgEntry.a == resEntry.a
            assert True if msgEntry.b is None else msgEntry.b == resEntry.b
        if msg.another is not None:
            assert True if msg.another.a is None else msg.another.a == res.another.a
            assert True if msg.another.b is None else msg.another.b == res.another.b


@pytest.mark.asyncio
async def test_failing_assert_can_stream():
    stream = b.stream.StreamFailingAssertion("Yoshimi battles the pink robots", 300)
    async for msg in stream:
        print(msg.story_a)
        print(msg.story_b)
    with pytest.raises(BamlValidationError):
        final = await stream.get_final_response()
        assert "Yoshimi" in final.story_a