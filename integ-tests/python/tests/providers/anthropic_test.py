import pytest
from assertpy import assert_that
from ..test_setup import b

@pytest.mark.asyncio
async def test_anthropic_shorthand():
    res = await b.TestAnthropicShorthand(input="Mt Rainier is tall")
    assert len(res) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_anthropic_shorthand_streaming():
    res = await b.stream.TestAnthropicShorthand(
        input="Mt Rainier is tall"
    ).get_final_response()
    assert len(res) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_claude():
    res = await b.PromptTestClaude(input="Mt Rainier is tall")
    assert len(res) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_streaming_claude():
    stream = b.stream.PromptTestClaude(input="Mt Rainier is tall")
    msgs = []
    async for msg in stream:
        msgs.append(str(msg))
    final = await stream.get_final_response()

    assert len(final) > 0, "Expected non-empty final but got empty."
    assert len(msgs) > 0, "Expected at least one streamed response but got none."
    for prev_msg, msg in zip(msgs, msgs[1:]):
        assert msg.startswith(prev_msg), (
            "Expected messages to be continuous, but prev was %r and next was %r"
            % (prev_msg, msg,)
        )
    assert msgs[-1] == final, "Expected last stream message to match final response."