use std::collections::VecDeque;

use anyhow::Result;

use crate::{
    deserializer::{
        deserialize_flags::{DeserializerConditions, Flag},
        types::BamlValueWithFlags,
    },
    jsonish,
};
use baml_types::{BamlMap, FieldType, LiteralValue, TypeValue};

use super::{ParsingContext, ParsingError, TypeCoercer};

pub(super) fn coerce_map(
    ctx: &ParsingContext,
    map_target: &FieldType,
    value: Option<&jsonish::Value>,
) -> Result<BamlValueWithFlags, ParsingError> {
    log::debug!(
        "scope: {scope} :: coercing to: {name} (current: {current})",
        name = map_target.to_string(),
        scope = ctx.display_scope(),
        current = value.map(|v| v.r#type()).unwrap_or("<null>".into())
    );

    let Some(value) = value else {
        return Err(ctx.error_unexpected_null(map_target));
    };

    let FieldType::Map(key_type, value_type) = map_target else {
        return Err(ctx.error_unexpected_type(map_target, value));
    };

    // TODO: Do we actually need to check the key type here in the coercion
    // logic? Can the user pass a "type" here at runtime? Can we pass the wrong
    // type from our own code or is this guaranteed to be a valid map key type?
    // If we can determine that the type is always valid then we can get rid of
    // this logic and skip the loops & allocs in the the union branch.
    match key_type.as_ref() {
        // String, int, enum or just one literal string or int, OK.
        FieldType::Primitive(TypeValue::String)
        | FieldType::Primitive(TypeValue::Int)
        | FieldType::Enum(_)
        | FieldType::Literal(LiteralValue::String(_))
        | FieldType::Literal(LiteralValue::Int(_)) => {}

        // For unions we need to check if all the items are literals of the same
        // type.
        FieldType::Union(items) => {
            let mut queue = VecDeque::from_iter(items.iter());

            // Same trick used in `validate_type_allowed` at
            // baml-lib/baml-core/src/validate/validation_pipeline/validations/types.rs
            //
            // TODO: Figure out how to reuse this.
            let mut literal_types_found = [0, 0, 0];
            let [strings, ints, bools] = &mut literal_types_found;

            while let Some(item) = queue.pop_front() {
                match item {
                    FieldType::Literal(LiteralValue::String(_)) => *strings += 1,
                    FieldType::Literal(LiteralValue::Int(_)) => *ints += 1,
                    FieldType::Literal(LiteralValue::Bool(_)) => *bools += 1,
                    FieldType::Union(nested) => queue.extend(nested.iter()),
                    other => return Err(ctx.error_map_must_have_supported_key(other)),
                }
            }

            if literal_types_found.iter().filter(|&&t| t > 0).count() > 1 {
                return Err(ctx.error_map_must_have_only_one_type_in_key_union(key_type));
            }
        }

        // Key type not allowed.
        other => return Err(ctx.error_map_must_have_supported_key(other)),
    }

    let mut flags = DeserializerConditions::new();
    flags.add_flag(Flag::ObjectToMap(value.clone()));

    match &value {
        jsonish::Value::Object(obj) => {
            let mut items = BamlMap::new();
            for (idx, (key, value)) in obj.iter().enumerate() {
                let coerced_value =
                    match value_type.coerce(&ctx.enter_scope(key), value_type, Some(value)) {
                        Ok(v) => v,
                        Err(e) => {
                            flags.add_flag(Flag::MapValueParseError(key.clone(), e));
                            // Could not coerce value, nothing else to do here.
                            continue;
                        }
                    };

                // Keys are just strings but since we suport enums and literals
                // we have to check that the key we are reading is actually a
                // valid enum member or expected literal value. The coercion
                // logic already does that so we'll just coerce the key.
                //
                // TODO: Is it necessary to check that values match here? This
                // is also checked at `coerce_arg` in
                // baml-lib/baml-core/src/ir/ir_helpers/to_baml_arg.rs
                let key_as_jsonish = jsonish::Value::String(key.to_owned());
                match key_type.coerce(ctx, &key_type, Some(&key_as_jsonish)) {
                    Ok(_) => {
                        // Hack to avoid cloning the key twice.
                        let jsonish::Value::String(owned_key) = key_as_jsonish else {
                            unreachable!("key_as_jsonish is defined as jsonish::Value::String");
                        };

                        // Both the value and the key were successfully
                        // coerced, add the key to the map.
                        items.insert(owned_key, (DeserializerConditions::new(), coerced_value));
                    }
                    // Couldn't coerce key, this is either not a valid enum
                    // variant or it doesn't match any of the literal values
                    // expected.
                    Err(e) => flags.add_flag(Flag::MapKeyParseError(idx, e)),
                }
            }
            Ok(BamlValueWithFlags::Map(flags, items))
        }
        // TODO: first map in an array that matches
        _ => Err(ctx.error_unexpected_type(map_target, value)),
    }
}
