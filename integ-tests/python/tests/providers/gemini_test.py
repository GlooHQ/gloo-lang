import pytest
from assertpy import assert_that
from ..test_setup import b

@pytest.mark.asyncio
async def test_gemini():
    geminiRes = await b.TestGemini(input="Dr. Pepper")
    print(f"LLM output from Gemini: {geminiRes}")
    assert len(geminiRes) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_gemini_streaming():
    geminiRes = await b.stream.TestGemini(input="Dr. Pepper").get_final_response()
    print(f"LLM output from Gemini: {geminiRes}")
    assert len(geminiRes) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_streaming_gemini():
    stream = b.stream.TestGemini(input="Dr.Pepper")
    msgs = []
    async for msg in stream:
        if msg is not None:
            msgs.append(msg)
    final = await stream.get_final_response()

    assert len(final) > 0, "Expected non-empty final but got empty."
    assert len(msgs) > 0, "Expected at least one streamed response but got none."
    for prev_msg, msg in zip(msgs, msgs[1:]):
        assert msg.startswith(prev_msg), (
            "Expected messages to be continuous, but prev was %r and next was %r"
            % (prev_msg, msg,)
        )
    assert msgs[-1] == final, "Expected last stream message to match final response."