import pytest
import json
import os
from assertpy import assert_that
from baml_py.type_builder import TypeBuilder
from baml_py import ClientRegistry
from .test_setup import b
from ..baml_client.types import (
    DynInputOutput,
    Hobby,
)

@pytest.mark.asyncio
async def test_dynamic():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.Person = tb.add_class("Person")
    tb.Person.add_property("last_name", tb.string().list())
    tb.Person.add_property("height", tb.float().optional()).description(
        "Height in meters"
    )

    tb.Hobby = tb.add_enum("Hobby")
    tb.Hobby.add_value("chess")
    for name, val in tb.Hobby.list_values():
        val.alias(name.lower())

    tb.Person.add_property("hobbies", tb.Hobby.type().list()).description(
        "Some suggested hobbies they might be good at"
    )

    tb_res = await b.ExtractPeople(
        "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop.",
        {"tb": tb},
    )

    assert len(tb_res) > 0, "Expected non-empty result but got empty."

    for r in tb_res:
        print(r.model_dump())


@pytest.mark.asyncio
async def test_dynamic_class_output():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.DynamicOutput = tb.add_class("DynamicOutput")
    tb.DynamicOutput.add_property("hair_color", tb.string())
    print(tb.DynamicOutput.list_properties())
    for prop in tb.DynamicOutput.list_properties():
        print(f"Property: {prop}")

    output = await b.MyFunc(
        input="My name is Harrison. My hair is black and I'm 6 feet tall.",
        baml_options={"tb": tb},
    )
    output = await b.MyFunc(
        input="My name is Harrison. My hair is black and I'm 6 feet tall.",
        baml_options={"tb": tb},
    )
    print(output.model_dump_json())
    assert output.hair_color == "black"  # type: ignore (dynamic property)


@pytest.mark.asyncio
async def test_dynamic_class_nested_output_no_stream():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.DynamicOutput = tb.add_class("DynamicOutput")
    tb.DynamicOutput.add_property("hair_color", tb.string())

    name_class = tb.add_class("Name")
    name_class.add_property("first_name", tb.string())
    name_class.add_property("last_name", tb.string().optional())
    name_class.add_property("middle_name", tb.string().optional())

    other_nested_class = tb.add_class("Address")
    other_nested_class.add_property("street", tb.string())
    other_nested_class.add_property("city", tb.string())
    other_nested_class.add_property("state", tb.string())
    other_nested_class.add_property("zip", tb.string())

    # name should be first in the prompt schema
    tb.DynamicOutput.add_property("name", name_class.type().optional())
    tb.DynamicOutput.add_property("address", other_nested_class.type().optional())
    tb.DynamicOutput.add_property("hair_color", tb.string()).alias("hairColor")
    tb.DynamicOutput.add_property("height", tb.float().optional())

    output = await b.MyFunc(
        input="My name is Mark Gonzalez. My hair is black and I'm 6 feet tall.",
        baml_options={"tb": tb},
    )
    print(output.model_dump_json())
    # Compare JSON objects instead of strings
    actual = json.loads(output.model_dump_json())
    expected = {
        "name": {"first_name": "Mark", "last_name": "Gonzalez", "middle_name": None},
        "address": None,
        "hair_color": "black",
        "height": 6.0
    }
    assert actual == expected


@pytest.mark.asyncio
async def test_dynamic_class_nested_output_stream():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.DynamicOutput = tb.add_class("DynamicOutput")
    tb.DynamicOutput.add_property("hair_color", tb.string())

    nested_class = tb.add_class("Name")
    nested_class.add_property("first_name", tb.string())
    nested_class.add_property("last_name", tb.string().optional())

    # name should be first in the prompt schema
    tb.DynamicOutput.add_property("name", nested_class.type().optional())
    tb.DynamicOutput.add_property("hair_color", tb.string())

    stream = b.stream.MyFunc(
        input="My name is Mark Gonzalez. My hair is black and I'm 6 feet tall.",
        baml_options={"tb": tb},
    )
    msgs = []
    async for msg in stream:
        print("streamed ", msg)
        print("streamed ", msg.model_dump())
        msgs.append(msg)
    output = await stream.get_final_response()

    print(output.model_dump_json())
    # Compare JSON objects instead of strings
    actual = json.loads(output.model_dump_json())
    expected = {
        "name": {"first_name": "Mark", "last_name": "Gonzalez"},
        "hair_color": "black"
    }
    assert actual == expected


@pytest.mark.asyncio
async def test_stream_dynamic_class_output():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.DynamicOutput = tb.add_class("DynamicOutput")
    tb.DynamicOutput.add_property("hair_color", tb.string())

    cr = ClientRegistry()
    cr.add_llm_client("MyClient", "openai", {"model": "gpt-4o-mini"})
    cr.set_primary("MyClient")
    stream = b.stream.MyFunc(
        input="My name is Harrison. My hair is black and I'm 6 feet tall.",
        baml_options={"tb": tb, "client_registry": cr},
    )
    msgs = []
    async for msg in stream:
        print("streamed ", msg.model_dump())
        msgs.append(msg)
    final = await stream.get_final_response()

    assert len(msgs) > 0, "Expected at least one streamed response but got none."
    print("final ", final)
    print("final ", final.model_dump())
    print("final ", final.model_dump_json())
    assert final.hair_color == "black"  # type: ignore (dynamic property)


@pytest.mark.asyncio
async def test_dynamic_inputs_list2():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.DynamicOutput = tb.add_class("DynamicOutput")
    tb.DynamicOutput.add_property("hair_color", tb.string())

    tb.DynInputOutput = tb.add_class("DynInputOutput")
    tb.DynInputOutput.add_property("new_key", tb.string().optional())
    custom_class = tb.add_class("MyBlah")
    custom_class.add_property("nestedKey1", tb.string())
    tb.DynInputOutput.add_property("blah", custom_class.type())

    res = await b.DynamicListInputOutput(
        [
            DynInputOutput.model_validate(
                {
                    "new_key": "hi1",
                    "testKey": "myTest",
                    "blah": {
                        "nestedKey1": "nestedVal",
                    },
                }
            ),
            DynInputOutput.model_validate(
                {
                    "new_key": "hi",
                    "testKey": "myTest",
                    "blah": {
                        "nestedKey1": "nestedVal",
                    },
                }
            ),
        ],
        {"tb": tb},
    )
    assert res[0].new_key == "hi1"  # type: ignore (dynamic property)
    assert res[0].testKey == "myTest"
    assert res[0].blah["nestedKey1"] == "nestedVal"  # type: ignore (dynamic property)
    assert res[1].new_key == "hi"  # type: ignore (dynamic property)
    assert res[1].testKey == "myTest"
    assert res[1].blah["nestedKey1"] == "nestedVal"  # type: ignore (dynamic property)


@pytest.mark.asyncio
async def test_dynamic_types_new_enum():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.Person = tb.add_class("Person")
    field_enum = tb.add_enum("Animal")
    animals = ["giraffe", "elephant", "lion"]
    for animal in animals:
        field_enum.add_value(animal.upper())
    tb.Person.add_property("animalLiked", field_enum.type())
    res = await b.ExtractPeople(
        "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop. I like giraffes.",
        {"tb": tb},
    )
    assert len(res) > 0
    assert res[0].animalLiked == "GIRAFFE", res[0]


@pytest.mark.asyncio
async def test_dynamic_types_existing_enum():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.Hobby = tb.add_enum("Hobby")
    tb.Hobby.add_value("Golfing")
    tb.Hobby.add_value("SPORTS")
    tb.Hobby.add_value("MUSIC")
    tb.Hobby.add_value("READING")
    res = await b.ExtractHobby(
        "My name is Harrison. My hair is black and I'm 6 feet tall. golf and music are my favorite!.",
        {"tb": tb},
    )
    assert len(res) > 0
    assert "Golfing" in res, res
    assert Hobby.MUSIC in res, res


@pytest.mark.asyncio
async def test_dynamic_literals():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.Person = tb.add_class("Person")
    animals = tb.union(
        [
            tb.literal_string(animal.upper())
            for animal in ["giraffe", "elephant", "lion"]
        ]
    )
    tb.Person.add_property("animalLiked", animals)
    res = await b.ExtractPeople(
        "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop. I like giraffes.",
        {"tb": tb},
    )
    assert len(res) > 0
    assert res[0].animalLiked == "GIRAFFE"


@pytest.mark.asyncio
async def test_dynamic_output_map():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.DynamicOutput = tb.add_class("DynamicOutput")
    tb.DynamicOutput.add_property("hair_color", tb.string())
    tb.DynamicOutput.add_property(
        "attributes", tb.map(tb.string(), tb.string())
    ).description("Things like 'eye_color' or 'facial_hair'")
    print(tb.DynamicOutput.list_properties())
    for prop, _ in tb.DynamicOutput.list_properties():
        print(f"Property: {prop}")

    res = await b.MyFunc(
        input="My name is Harrison. My hair is black and I'm 6 feet tall. I have blue eyes and a beard.",
        baml_options={"tb": tb},
    )

    print("final ", res)
    print("final ", res.model_dump())
    print("final ", res.model_dump_json())
    assert res.hair_color == "black"  # type: ignore (dynamic property)
    assert res.attributes["eye_color"] == "blue"  # type: ignore (dynamic property)
    assert res.attributes["facial_hair"] == "beard"  # type: ignore (dynamic property)


@pytest.mark.asyncio
async def test_dynamic_output_union():
    tb = TypeBuilder(classes=set(), enums=set())
    tb.DynamicOutput = tb.add_class("DynamicOutput")
    tb.DynamicOutput.add_property("hair_color", tb.string())
    tb.DynamicOutput.add_property(
        "attributes", tb.map(tb.string(), tb.string())
    ).description("Things like 'eye_color' or 'facial_hair'")
    # Define two classes
    class1 = tb.add_class("Class1")
    class1.add_property("meters", tb.float())

    class2 = tb.add_class("Class2")
    class2.add_property("feet", tb.float())
    class2.add_property("inches", tb.float().optional())

    # Use the classes in a union property
    tb.DynamicOutput.add_property("height", tb.union([class1.type(), class2.type()]))
    print(tb.DynamicOutput.list_properties())
    for prop, _ in tb.DynamicOutput.list_properties():
        print(f"Property: {prop}")

    res = await b.MyFunc(
        input="My name is Harrison. My hair is black and I'm 6 feet tall. I have blue eyes and a beard. I am 30 years old.",
        baml_options={"tb": tb},
    )

    print("final ", res)
    print("final ", res.model_dump())
    print("final ", res.model_dump_json())
    assert res.hair_color == "black"  # type: ignore (dynamic property)
    assert res.attributes["eye_color"] == "blue"  # type: ignore (dynamic property)
    assert res.attributes["facial_hair"] == "beard"  # type: ignore (dynamic property)
    assert res.height["feet"] == 6  # type: ignore (dynamic property)

    res = await b.MyFunc(
        input="My name is Harrison. My hair is black and I'm 1.8 meters tall. I have blue eyes and a beard. I am 30 years old.",
        baml_options={"tb": tb},
    )

    print("final ", res)
    print("final ", res.model_dump())
    print("final ", res.model_dump_json())
    assert res.hair_color == "black"  # type: ignore (dynamic property)
    assert res.attributes["eye_color"] == "blue"  # type: ignore (dynamic property)
    assert res.attributes["facial_hair"] == "beard"  # type: ignore (dynamic property)
    assert res.height["meters"] == 1.8  # type: ignore (dynamic property)


@pytest.mark.asyncio
async def test_dynamic_client_with_openai():
    tb = TypeBuilder(classes=set(), enums=set())
    cb = ClientRegistry()
    cb.add_llm_client("MyClient", "openai", {"model": "gpt-3.5-turbo"})
    cb.set_primary("MyClient")

    capitol = await b.ExpectFailure(
        baml_options={"client_registry": cb},
    )
    assert_that(capitol.lower()).contains("london")


@pytest.mark.asyncio
async def test_dynamic_client_with_vertex_json_str_creds():
    tb = TypeBuilder(classes=set(), enums=set())
    cb = ClientRegistry()
    cb.add_llm_client(
        "MyClient",
        "vertex-ai",
        {
            "model": "gemini-1.5-pro",
            "location": "us-central1",
            "credentials": os.environ[
                "INTEG_TESTS_GOOGLE_APPLICATION_CREDENTIALS_CONTENT"
            ],
        },
    )
    cb.set_primary("MyClient")

    capitol = await b.ExpectFailure(
        baml_options={"client_registry": cb},
    )
    assert_that(capitol.lower()).contains("london")


@pytest.mark.asyncio
async def test_dynamic_client_with_vertex_json_object_creds():
    tb = TypeBuilder(classes=set(), enums=set())
    cb = ClientRegistry()
    cb.add_llm_client(
        "MyClient",
        "vertex-ai",
        {
            "model": "gemini-1.5-pro",
            "location": "us-central1",
            "credentials": json.loads(
                os.environ["INTEG_TESTS_GOOGLE_APPLICATION_CREDENTIALS_CONTENT"]
            ),
        },
    )
    cb.set_primary("MyClient")

    capitol = await b.ExpectFailure(
        baml_options={"client_registry": cb},
    )
    assert_that(capitol.lower()).contains("london")