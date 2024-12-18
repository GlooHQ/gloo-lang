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
    test_recursive_alias_union,
    r#"
type JsonValue = int | string | bool | JsonValue[] | map<string, JsonValue>
    "#,
    r#"
    {
        "number": 1,
        "string": "test",
        "bool": true
    }
    "#,
    FieldType::RecursiveTypeAlias("JsonValue".into()),
    {
        "number": 1,
        "string": "test",
        "bool": true
    }
);

test_deserializer!(
    test_complex_recursive_alias,
    r#"
type JsonValue = int | string | bool | JsonValue[] | map<string, JsonValue>
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
