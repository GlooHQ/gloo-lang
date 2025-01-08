import os
import pytest
from dotenv import load_dotenv
import baml_py
from ..baml_client import b
from ..baml_client.sync_client import b as sync_b
from ..baml_client.globals import (
    DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME,
)
from ..baml_client.tracing import flush

load_dotenv()

@pytest.fixture(scope="session", autouse=True)
def cleanup():
    """Cleanup a testing directory once we are finished."""
    flush()