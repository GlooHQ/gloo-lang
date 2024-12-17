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
