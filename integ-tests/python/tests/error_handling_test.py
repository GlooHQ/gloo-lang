import pytest
from assertpy import assert_that
from baml_py import errors, ClientRegistry
from .test_setup import b

@pytest.mark.asyncio
async def test_serialization_exception():
    with pytest.raises(Exception) as excinfo:
        await b.DummyOutputFunction("dummy input")

    print("Exception message from test: ", excinfo)
    assert "Failed to coerce" in str(excinfo)


@pytest.mark.asyncio
async def test_stream_serialization_exception():
    with pytest.raises(Exception) as excinfo:
        stream = b.stream.DummyOutputFunction("dummy input")
        async for msg in stream:
            print("streamed ", msg)

        _ = await stream.get_final_response()

    print("Exception message: ", excinfo)
    assert "Failed to coerce" in str(excinfo)


@pytest.mark.asyncio
async def test_arg_exceptions():
    with pytest.raises(IndexError):
        print("this should fail:", [0, 1, 2][5])

    with pytest.raises(errors.BamlInvalidArgumentError):
        _ = await b.TestCaching(
            111,  # type: ignore -- intentionally passing an int instead of a string
            "..",
        )

    with pytest.raises(errors.BamlClientError):
        cr = ClientRegistry()
        cr.add_llm_client(
            "MyClient", "openai", {"model": "gpt-4o-mini", "api_key": "INVALID_KEY"}
        )
        cr.set_primary("MyClient")
        await b.MyFunc(
            input="My name is Harrison. My hair is black and I'm 6 feet tall.",
            baml_options={"client_registry": cr},
        )

    with pytest.raises(errors.BamlClientHttpError):
        cr = ClientRegistry()
        cr.add_llm_client(
            "MyClient", "openai", {"model": "gpt-4o-mini", "api_key": "INVALID_KEY"}
        )
        cr.set_primary("MyClient")
        await b.MyFunc(
            input="My name is Harrison. My hair is black and I'm 6 feet tall.",
            baml_options={"client_registry": cr},
        )

    with pytest.raises(errors.BamlValidationError):
        await b.DummyOutputFunction("dummy input")


@pytest.mark.asyncio
async def test_map_as_param():
    with pytest.raises(errors.BamlInvalidArgumentError):
        _ = await b.TestFnNamedArgsSingleMapStringToMap(
            {"a": "b"}
        )  # intentionally passing the wrong type


@pytest.mark.asyncio
async def test_baml_validation_error_format():
    with pytest.raises(errors.BamlValidationError) as excinfo:
        try:
            await b.DummyOutputFunction("blah")
        except errors.BamlValidationError as e:
            print("Error: ", e)
            assert hasattr(e, "prompt"), "Error object should have 'prompt' attribute"
            assert hasattr(
                e, "raw_output"
            ), "Error object should have 'raw_output' attribute"
            assert hasattr(e, "message"), "Error object should have 'message' attribute"
            assert 'Say "hello there"' in e.prompt

            raise e
    assert "Failed to parse" in str(excinfo)