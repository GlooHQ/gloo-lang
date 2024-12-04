import baml_py
import inspect
import pydantic
import pytest


class Foo(pydantic.BaseModel):
    my_image: baml_py.Image


def test_model_validate_success():
    foo_inst = Foo.model_validate(
        {"my_image": {"url": "https://example.com/image.png"}}
    )
    assert isinstance(foo_inst.my_image, baml_py.Image)

    foo_inst = Foo.model_validate(
        {"my_image": {"url": "https://example.com/image.png", "media_type": None}}
    )
    assert isinstance(foo_inst.my_image, baml_py.Image)

    foo_inst = Foo.model_validate(
        {
            "my_image": {
                "url": "https://example.com/image.png",
                "media_type": "image/png",
            }
        }
    )
    assert isinstance(foo_inst.my_image, baml_py.Image)

    foo_inst = Foo.model_validate(
        {"my_image": {"base64": "iVBORw0KGgoAAAANSUhEUgAAAAUA"}}
    )
    assert isinstance(foo_inst.my_image, baml_py.Image)

    foo_inst = Foo.model_validate(
        {
            "my_image": {
                "base64": "iVBORw0KGgoAAAANSUhEUgAAAAUA",
                "media_type": None,
            }
        }
    )
    assert isinstance(foo_inst.my_image, baml_py.Image)

    foo_inst = Foo.model_validate(
        {
            "my_image": {
                "base64": "iVBORw0KGgoAAAANSUhEUgAAAAUA",
                "media_type": "image/png",
            }
        }
    )
    assert isinstance(foo_inst.my_image, baml_py.Image)


def test_model_validate_failure():
    # assert that model validation produces a useful error
    with pytest.raises(pydantic.ValidationError) as e:
        Foo.model_validate({"my_image": {"not-a-url": "https://example.com/image.png"}})
        assert "my_image" in str(e.value)
        assert "base64" in str(e.value)
        assert "url" in str(e.value)


def test_model_dump():
    foo_inst = Foo(my_image=baml_py.Image.from_url("https://example.com/image.png"))
    assert foo_inst.model_dump() == {
        "my_image": {"url": "https://example.com/image.png"}
    }

    foo_inst = Foo(
        my_image=baml_py.Image.from_base64("image/png", "iVBORw0KGgoAAAANSUhEUgAAAAUA")
    )
    assert foo_inst.model_dump() == {
        "my_image": {
            "base64": "iVBORw0KGgoAAAANSUhEUgAAAAUA",
            "media_type": "image/png",
        }
    }


def test_inspect():
    """Assert that baml_py is compatible with the inspect module.

    This is a regression test for a bug where `inspect.stack()` would implode if the
    pyo3 code called `PyModule::from_code` without specifying the `file_name` arg (i.e.
    without specifying the source file metadata for the inline Python snippet).
    """

    class LoremIpsum(pydantic.BaseModel):  # pyright: ignore[reportUnusedClass]
        """Defining this Pydantic model alone is sufficient to trigger the bug."""

        my_image: baml_py.Image
        my_audio: baml_py.Audio

    try:
        inspect.stack()
    except Exception as e:
        pytest.fail(f"inspect.stack() raised an unexpected exception: {e}")
