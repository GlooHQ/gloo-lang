import pytest
from assertpy import assert_that
from ..test_setup import b

@pytest.mark.asyncio
async def test_openai_shorthand():
    res = await b.TestOpenAIShorthand(input="Mt Rainier is tall")
    assert len(res) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_openai_shorthand_streaming():
    res = await b.stream.TestOpenAIShorthand(
        input="Mt Rainier is tall"
    ).get_final_response()
    assert len(res) > 0, "Expected non-empty result but got empty."

