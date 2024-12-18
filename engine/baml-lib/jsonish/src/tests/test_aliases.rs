use baml_types::LiteralValue;

use super::*;

test_deserializer!(
    test_simple_recursive_alias_list,
    r#"
type A = A[]
    "#,
    "[[], [], [[]]]",
    FieldType::RecursiveTypeAlias("A".into()),
    [[], [], [[]]]
);

test_deserializer!(
    test_simple_recursive_alias_map,
    r#"
type A = map<string, A>
    "#,
    r#"{"one": {"two": {}}, "three": {"four": {}}}"#,
    FieldType::RecursiveTypeAlias("A".into()),
    {
        "one": {"two": {}},
        "three": {"four": {}}
    }
);

test_deserializer!(
    test_recursive_alias_cycle,
    r#"
type A = B
type B = C
type C = A[]
    "#,
    "[[], [], [[]]]",
    FieldType::RecursiveTypeAlias("A".into()),
    [[], [], [[]]]
);

test_deserializer!(
    test_json_without_nested_objects,
    r#"
type JsonValue = int | float | bool | string | null | JsonValue[] | map<string, JsonValue> 
    "#,
    r#"
    {
        "int": 1,
        "float": 1.0,
        "string": "test",
        "bool": true
    }
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    {
        "int": 1,
        "float": 1.0,
        "string": "test",
        "bool": true
    }
);

test_deserializer!(
    test_json_with_nested_list,
    r#"
type JsonValue = int | float | bool | string | null | JsonValue[] | map<string, JsonValue> 
    "#,
    r#"
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "list": [1, 2, 3]
    }
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "list": [1, 2, 3]
    }
);

test_deserializer!(
    test_json_with_nested_object,
    r#"
type JsonValue = int | float | bool | string | null | JsonValue[] | map<string, JsonValue> 
    "#,
    r#"
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "json": {
            "number": 1,
            "string": "test",
            "bool": true
        }
    }
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "json": {
            "number": 1,
            "string": "test",
            "bool": true
        }
    }
);

test_deserializer!(
    test_full_json_with_nested_objects,
    r#"
type JsonValue = int | float | bool | string | null | JsonValue[] | map<string, JsonValue> 
    "#,
    r#"
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "list": [1, 2, 3],
        "object": {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3]
        },
        "json": {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3],
            "object": {
                "number": 1,
                "string": "test",
                "bool": true,
                "list": [1, 2, 3]
            }
        }
    }
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "list": [1, 2, 3],
        "object": {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3]
        },
        "json": {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3],
            "object": {
                "number": 1,
                "string": "test",
                "bool": true,
                "list": [1, 2, 3]
            }
        }
    }
);

test_deserializer!(
    test_list_of_json_objects,
    r#"
type JsonValue = int | float | bool | string | null | JsonValue[] | map<string, JsonValue> 
    "#,
    r#"
    [
        {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3]
        },
        {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3]
        }
    ]
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    [
        {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3]
        },
        {
            "number": 1,
            "string": "test",
            "bool": true,
            "list": [1, 2, 3]
        }
    ]
);

test_deserializer!(
    test_nested_list,
    r#"
type JsonValue = int | float | bool | string | null | JsonValue[] | map<string, JsonValue> 
    "#,
    r#"
    [[42.1]]
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    // [[[[[[[[[[[[[[[[[[[[42]]]]]]]]]]]]]]]]]]]]
    [[42.1]]
);

test_deserializer!(
    test_json_defined_with_cycles,
    r#"
type JsonValue = int | float | bool | string | null | JsonArray | JsonObject
type JsonArray = JsonValue[]
type JsonObject = map<string, JsonValue>
    "#,
    r#"
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "json": {
            "number": 1,
            "string": "test",
            "bool": true
        }
    }
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    {
        "number": 1,
        "string": "test",
        "bool": true,
        "json": {
            "number": 1,
            "string": "test",
            "bool": true
        }
    }
);
