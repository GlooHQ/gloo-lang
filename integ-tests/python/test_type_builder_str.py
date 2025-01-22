import pytest
from baml_client.type_builder import TypeBuilder

def test_type_builder_str():
    """
     test for typebuilder's string representation functionality.

    this test verifies that the typebuilder correctly represents its structure
    in string format, ensuring proper exposure of the rust implementation to python.

    test coverage:
    -------------
    1. class representation:
       - class names and structure
       - property definitions with types
       - property metadata:
         * aliases for alternative naming
         * descriptions for documentation

    2. enum representation:
       - enum names and structure
       - value definitions
       - value metadata:
         * aliases for alternative naming
         * descriptions for documentation

    3. cross-language integration:
       - verifies that the rust string representation is correctly
         exposed through the python bindings
       - ensures consistent formatting across language boundaries
    """
    # Create a new TypeBuilder
    tb = TypeBuilder()

    # Add a class with properties and metadata
    user = tb.class_("User")
    name_prop = user.property("name")
    name_prop.type(tb.string())
    name_prop.with_meta("alias", "username")
    name_prop.with_meta("description", "The user's full name")

    age_prop = user.property("age")
    age_prop.type(tb.int())
    age_prop.with_meta("description", "User's age in years")

    email_prop = user.property("email")
    email_prop.type(tb.string())

    # Add an enum with values and metadata
    status = tb.enum("Status")
    active = status.value("ACTIVE")
    active.with_meta("alias", "active")
    active.with_meta("description", "User is active")

    inactive = status.value("INACTIVE")
    inactive.with_meta("alias", "inactive")

    status.value("PENDING")

    # Convert to string and verify the format
    output = str(tb)
    print(f"TypeBuilder string representation:\n{output}")

    # Verify the expected format
    assert "User" in output
    assert "name" in output
    assert "username" in output
    assert "The user's full name" in output
    assert "age" in output
    assert "User's age in years" in output
    assert "email" in output
    assert "Status" in output
    assert "ACTIVE" in output
    assert "active" in output
    assert "User is active" in output
    assert "INACTIVE" in output
    assert "inactive" in output
    assert "PENDING" in output