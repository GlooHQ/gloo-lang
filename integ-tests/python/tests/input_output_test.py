import pytest
import datetime
from assertpy import assert_that
from baml_py import Image, Audio
from .test_setup import b
from ..baml_client.types import (
    NamedArgsSingleClass,
    StringToClassEntry,
    MapKey,
)
from .base64_test_data import image_b64, audio_b64

class MyCustomClass(NamedArgsSingleClass):
    date: datetime.datetime


@pytest.mark.asyncio
async def test_accepts_subclass_of_baml_type():
    print("calling with class")
    _ = await b.TestFnNamedArgsSingleClass(
        myArg=MyCustomClass(
            key="key", key_two=True, key_three=52, date=datetime.datetime.now()
        )
    )


@pytest.mark.asyncio
async def test_should_work_for_all_outputs():
    a = "a"  # dummy
    res = await b.FnOutputBool(a)
    assert res == True

    integer = await b.FnOutputInt(a)
    assert integer == 5

    literal_integer = await b.FnOutputLiteralInt(a)
    assert literal_integer == 5

    literal_bool = await b.FnOutputLiteralBool(a)
    assert literal_bool == False

    literal_string = await b.FnOutputLiteralString(a)
    assert literal_string == "example output"

    list = await b.FnOutputClassList(a)
    assert len(list) > 0
    assert len(list[0].prop1) > 0

    classWEnum = await b.FnOutputClassWithEnum(a)
    assert classWEnum.prop2 in ["ONE", "TWO"]

    classs = await b.FnOutputClass(a)
    assert classs.prop1 is not None
    assert classs.prop2 == 540

    enumList = await b.FnEnumListOutput(a)
    assert len(enumList) == 2

    myEnum = await b.FnEnumOutput(a)
    # As no check is added for myEnum, adding a simple assert to ensure the call was made
    assert myEnum is not None


@pytest.mark.asyncio
async def test_should_work_with_image_url():
    res = await b.TestImageInput(
        img=Image.from_url(
            "https://upload.wikimedia.org/wikipedia/en/4/4d/Shrek_%28character%29.png"
        )
    )
    assert_that(res.lower()).matches(r"(green|yellow|shrek|ogre)")


@pytest.mark.asyncio
async def test_should_work_with_image_list():
    res = await b.TestImageListInput(
        imgs=[
            Image.from_url(
                "https://upload.wikimedia.org/wikipedia/en/4/4d/Shrek_%28character%29.png"
            ),
            Image.from_url(
                "https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_92x30dp.png"
            ),
        ]
    )
    assert_that(res.lower()).matches(r"(green|yellow)")


@pytest.mark.asyncio
async def test_should_work_with_image_base64():
    res = await b.TestImageInput(img=Image.from_base64("image/png", image_b64))
    assert_that(res.lower()).matches(r"(green|yellow|shrek|ogre)")


@pytest.mark.asyncio
async def test_should_work_with_audio_base64():
    res = await b.AudioInput(aud=Audio.from_base64("audio/mp3", audio_b64))
    assert "yes" in res.lower()


@pytest.mark.asyncio
async def test_should_work_with_audio_url():
    res = await b.AudioInput(
        aud=Audio.from_url(
            "https://actions.google.com/sounds/v1/emergency/beeper_emergency_call.ogg"
        )
    )
    assert "no" in res.lower()


@pytest.mark.asyncio
async def test_single_bool():
    res = await b.TestFnNamedArgsSingleBool(True)
    assert res


@pytest.mark.asyncio
async def test_single_string_list():
    res = await b.TestFnNamedArgsSingleStringList(["a", "b", "c"])
    assert "a" in res and "b" in res and "c" in res


@pytest.mark.asyncio
async def test_return_literal_union():
    res = await b.LiteralUnionsTest("a")
    assert res == 1 or res == True or res == "string output"


@pytest.mark.asyncio
async def test_single_class():
    res = await b.TestFnNamedArgsSingleClass(
        myArg=NamedArgsSingleClass(
            key="key",
            key_two=True,
            key_three=52,
        )
    )
    assert "52" in res


@pytest.mark.asyncio
async def test_multiple_args():
    res = await b.TestMulticlassNamedArgs(
        myArg=NamedArgsSingleClass(
            key="key",
            key_two=True,
            key_three=52,
        ),
        myArg2=NamedArgsSingleClass(
            key="key",
            key_two=True,
            key_three=64,
        ),
    )
    assert "52" in res and "64" in res


@pytest.mark.asyncio
async def test_single_float():
    res = await b.TestFnNamedArgsSingleFloat(3.12)
    assert "3.12" in res


@pytest.mark.asyncio
async def test_single_int():
    res = await b.TestFnNamedArgsSingleInt(3566)
    assert "3566" in res


@pytest.mark.asyncio
async def test_single_literal_int():
    res = await b.TestNamedArgsLiteralInt(1)
    assert "1" in res


@pytest.mark.asyncio
async def test_single_literal_bool():
    res = await b.TestNamedArgsLiteralBool(True)
    assert "true" in res


@pytest.mark.asyncio
async def test_single_literal_string():
    res = await b.TestNamedArgsLiteralString("My String")
    assert "My String" in res


@pytest.mark.asyncio
async def test_single_map_string_to_string():
    res = await b.TestFnNamedArgsSingleMapStringToString(
        {"lorem": "ipsum", "dolor": "sit"}
    )
    assert "lorem" in res


@pytest.mark.asyncio
async def test_single_map_string_to_class():
    res = await b.TestFnNamedArgsSingleMapStringToClass(
        {"lorem": StringToClassEntry(word="ipsum")}
    )
    assert res["lorem"].word == "ipsum"


@pytest.mark.asyncio
async def test_single_map_string_to_map():
    res = await b.TestFnNamedArgsSingleMapStringToMap({"lorem": {"word": "ipsum"}})
    assert res["lorem"]["word"] == "ipsum"


@pytest.mark.asyncio
async def test_enum_key_in_map():
    res = await b.InOutEnumMapKey({MapKey.A: "A"}, {MapKey.B: "B"})
    assert res[MapKey.A] == "A"
    assert res[MapKey.B] == "B"


@pytest.mark.asyncio
async def test_literal_string_union_key_in_map():
    res = await b.InOutLiteralStringUnionMapKey({"one": "1"}, {"two": "2"})
    assert res["one"] == "1"
    assert res["two"] == "2"


@pytest.mark.asyncio
async def test_single_literal_string_key_in_map():
    res = await b.InOutSingleLiteralStringMapKey({"key": "1"})
    assert res["key"] == "1"