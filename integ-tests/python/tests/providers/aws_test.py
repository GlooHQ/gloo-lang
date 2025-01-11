import pytest
from assertpy import assert_that
from baml_py import errors
from ..test_setup import b

@pytest.mark.asyncio
async def test_aws():
    res = await b.TestAws(input="Mt Rainier is tall")
    assert len(res) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_aws_streaming():
    res = await b.stream.TestAws(input="Mt Rainier is tall").get_final_response()
    assert len(res) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_aws_bedrock():
    ## unstreamed
    res = await b.TestAws("lightning in a rock")
    print("unstreamed", res)

    ## streamed
    stream = b.stream.TestAws("lightning in a rock")

    async for msg in stream:
        if msg:
            print("streamed ", repr(msg[-100:]))

    res = await stream.get_final_response()
    print("streamed final", res)
    assert len(res) > 0, "Expected non-empty result but got empty."


@pytest.mark.asyncio
async def test_aws_bedrock_invalid_region():
    ## unstreamed
    with pytest.raises(errors.BamlClientError) as excinfo:
        res = await b.TestAwsInvalidRegion("lightning in a rock")
        print("unstreamed", res)

    assert "DispatchFailure" in str(excinfo)