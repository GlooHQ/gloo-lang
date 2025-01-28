// This module helps resolve baml values with attached streaming state
// in the context of the streaming behavior associated with their types.

use crate::deserializer::coercer::ParsingError;
use crate::{BamlValueWithFlags, Flag};
use indexmap::IndexMap;
use internal_baml_core::ir::repr::{IntermediateRepr, Walker};
use internal_baml_core::ir::{Field, IRHelper};

use baml_types::{
    BamlValueWithMeta, Completion, CompletionState, FieldType, ResponseCheck, StreamingBehavior, TypeValue,
};

use anyhow::{Context, Error};
use std::collections::{HashMap, HashSet};
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum StreamingError {
    #[error("Expected to encounter a class")]
    ExpectedClass,
    #[error("Value was marked Done, but was incomplete in the stream")]
    IncompleteDoneValue,
    #[error("Class instance did not contain fields marked as needed")]
    MissingNeededFields,
    #[error("Failed to distribute_type_with_meta: {0}")]
    DistributeTypeWithMetaFailure(#[from] anyhow::Error),
}

/// For a given baml value, traverse its nodes, comparing the completion state
/// of each node against the streaming behavior of the node's type.
pub fn validate_streaming_state(
    ir: &IntermediateRepr,
    baml_value: &BamlValueWithFlags,
    field_type: &FieldType,
    allow_partials: bool,
) -> Result<BamlValueWithMeta<Completion>, StreamingError> {
    let baml_value_with_meta_flags: BamlValueWithMeta<Vec<Flag>> = baml_value.clone().into();
    let typed_baml_value: BamlValueWithMeta<(Vec<Flag>, FieldType)> =
        ir.distribute_type_with_meta(baml_value_with_meta_flags, field_type.clone())?;
    let baml_value_with_streaming_state_and_behavior =
        typed_baml_value.map_meta(|(flags, r#type)| (completion_state(&flags), r#type));

    let res = process_node(
        ir,
        baml_value_with_streaming_state_and_behavior,
        allow_partials,
    );
    res
}

/// Consider a node's type, streaming state, and streaming behavior annotations. Return
/// an error if streaming state doesn't meet the streaming requirements. Also attach
/// the streaming state to the node as metadata, if this was requested by the user
/// vial `@stream.with_state`.
///
/// This function descends into child nodes when the argument is a compound value.
/// 
/// Params:
///   value: A done in the BamlValue tree.
///   allow_partials: 
fn process_node(
    ir: &IntermediateRepr,
    value: BamlValueWithMeta<(CompletionState, &FieldType)>,
    allow_partials: bool,
) -> Result<BamlValueWithMeta<Completion>, StreamingError> {
    // let value_copy = value.clone(); // For debugging later. Delete me.
    let (completion_state, field_type) = value.meta().clone();
    let (base_type, (_, streaming_behavior)) = ir.distribute_metadata(field_type);

    let must_be_done = required_done(ir, field_type);
    let allow_partials_in_sub_nodes = allow_partials && !must_be_done;

    let new_meta = Completion {
        state: completion_state.clone(),
        display: streaming_behavior.state,
        required_done: must_be_done,
    };

    if must_be_done && allow_partials && !(completion_state == CompletionState::Complete) {
        return Err(StreamingError::IncompleteDoneValue);
    }

    let new_value = match value {
        BamlValueWithMeta::String(s, _) => Ok(BamlValueWithMeta::String(s, new_meta)),
        BamlValueWithMeta::Media(m, _) => Ok(BamlValueWithMeta::Media(m, new_meta)),
        BamlValueWithMeta::Null(_) => Ok(BamlValueWithMeta::Null(new_meta)),
        BamlValueWithMeta::Int(i, _) => Ok(BamlValueWithMeta::Int(i, new_meta)),
        BamlValueWithMeta::Float(f, _) => Ok(BamlValueWithMeta::Float(f, new_meta)),
        BamlValueWithMeta::Bool(b, _) => Ok(BamlValueWithMeta::Bool(b, new_meta)),
        BamlValueWithMeta::List(items, _) => Ok(BamlValueWithMeta::List(
            items
                .into_iter()
                .filter_map(|item| process_node(ir, item, allow_partials_in_sub_nodes).ok())
                .collect(),
            new_meta,
        )),
        BamlValueWithMeta::Class(ref class_name, value_fields, _) => {
            let value_field_names: HashSet<String> = value_fields
                .keys()
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            let needed_fields: HashSet<String> = needed_fields(ir, field_type, allow_partials_in_sub_nodes)?;

            let present_nonnull_fields: HashSet<String> = value_fields.iter().filter_map(|(field_name, field_value)| {
                if matches!(field_value, BamlValueWithMeta::Null(_)) {
                    None
                } else {
                    Some(field_name.to_string())
                }
            }).collect();
            // let missing_needed_fields = needed_fields.difference(&new_field_names);
            let missing_needed_fields: Vec<_> = needed_fields.difference(&present_nonnull_fields).into_iter().collect();
            // if (class_name == "SmallThing") {
            if false {
                dbg!(class_name);
                dbg!(&value_field_names);
                dbg!(&present_nonnull_fields);
                dbg!(&needed_fields);
                dbg!(&missing_needed_fields);
                dbg!(&value_fields);
            }


            // The fields that need to be filled in by Null are initially the
            // fields in the Class type that are not present in the input
            // value.
            let fields_needing_null =
                fields_needing_null_filler(ir, field_type, value_field_names, allow_partials)?;

            let mut deleted_fields: HashMap<String, BamlValueWithMeta<Completion>> =
                HashMap::new();

            // let unneeded_fields = field_names.difference(&needed_fields);
            let needed_nulls = fields_needing_null
                .into_iter()
                .filter_map(|ref null_field_name| {
                    let field = value_fields
                        .get(null_field_name)
                        .expect("This field is guaranteed to be in the field set");
                    let use_state = type_streaming_behavior(ir, field.meta().1).state;
                    let field_stream_state = Completion {
                        state: CompletionState::Incomplete,
                        display: use_state,
                        required_done: false,
                    };
                    Some((
                        null_field_name.to_string(),
                        BamlValueWithMeta::Null(field_stream_state),
                    ))
                })
                .collect::<IndexMap<String, BamlValueWithMeta<Completion>>>();

            let mut new_fields = value_fields
                .into_iter()
                .filter_map(|(field_name, field_value)| {
                    let with_state = field_value
                        .meta()
                        .1
                        .streaming_behavior()
                        .as_ref()
                        .map_or(false, |b| b.state);
                    let completion_state = field_value.meta().0.clone();
                    match process_node(ir, field_value, allow_partials_in_sub_nodes) {
                        Ok(res) => Some((field_name, res)),
                        _ => {
                            let state = Completion {
                                state: completion_state,
                                display: with_state,
                                required_done: false,
                            };
                            let null = BamlValueWithMeta::Null(state);
                            deleted_fields.insert(field_name, null);
                            None
                        }
                    }
                })
                .collect::<IndexMap<String, BamlValueWithMeta<_>>>();

            let derived_present_nonnull_fields: HashSet<String> = new_fields.iter().filter_map(|(field_name, field_value)| {
                if matches!(field_value, BamlValueWithMeta::Null(_)) {
                    None
                } else {
                    Some(field_name.to_string())
                }
            }).collect();
            let missing_needed_fields: Vec<_> = needed_fields.difference(&derived_present_nonnull_fields).into_iter().collect();

            new_fields.extend(needed_nulls);
            new_fields.extend(deleted_fields);

            let res = BamlValueWithMeta::Class(class_name.clone(), new_fields, new_meta);
            if missing_needed_fields.clone().len() == 0 {
                Ok(res)
            } else {
                Err(StreamingError::MissingNeededFields)
            }
        }
        BamlValueWithMeta::Enum(name, value, _) => {
            Ok(BamlValueWithMeta::Enum(name, value, new_meta))
        }
        BamlValueWithMeta::Map(kvs, _) => {
            let new_kvs = kvs
                .into_iter()
                .filter_map(|(k, v)| process_node(ir, v, allow_partials_in_sub_nodes).ok().map(|v| (k, v)))
                .collect();
            Ok(BamlValueWithMeta::Map(new_kvs, new_meta))
        }
    };

    new_value
}


fn fields_needing_null_filler<'a>(
    ir: &'a IntermediateRepr,
    field_type: &'a FieldType,
    value_names: HashSet<String>,
    allow_partials: bool,
) -> Result<HashSet<String>, anyhow::Error> {
    if allow_partials == false {
        return Ok(HashSet::new());
    }
    let res = match ir.distribute_metadata(field_type).0 {
        FieldType::Class(class_name) => match ir.find_class(class_name) {
            Err(_) => Ok(HashSet::new()),
            Ok(class) => {
                let missing_fields = class
                    .walk_fields()
                    .filter_map(|field: Walker<'_, &Field>| {
                        if !value_names.contains(field.name()) {
                            Some(field.name().to_string())
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(missing_fields)
            }
        },
        _ => Err(StreamingError::ExpectedClass).context(format!(
            "needed_fields expected Class, got type {field_type:?}"
        )),
    };
    res
}

/// For a given type, assume that it is a class, and list the fields of that
/// class that were marked `@stream.not_null`.
/// The parameter must have already been passed through `distribute_metadata`,
/// it's an error to call this function with undistributed metadata.
///
/// When allow_partials==false, we are in a context where we are done with
/// streaming, so we override the normal implemenation of this function
/// and return an empty set (because we are ignoring the "needed" property,
/// which only applies to mid-stream messages).
fn needed_fields(
    ir: &IntermediateRepr,
    field_type: &FieldType,
    allow_partials: bool,
) -> Result<HashSet<String>, anyhow::Error> {
    if allow_partials == false {
        return Ok(HashSet::new());
    }
    match ir.distribute_metadata(field_type).0 {
        FieldType::Class(class_name) => {
            let class = ir
                .find_class(class_name)
                .map_err(|_| StreamingError::ExpectedClass)
                .context("needed_fields failed to lookup class")?;
            let needed_fields = class
                .walk_fields()
                .filter_map(|field: Walker<'_, &Field>| {
                    if field.streaming_needed() {
                        Some(field.name().to_string())
                    } else {
                        None
                    }
                })
                .collect();
            Ok(needed_fields)
        }
        _ => Err(StreamingError::ExpectedClass).context(format!(
            "needed_fields expected Class got field type {field_type:?}"
        )), // TODO: Handle type aliases?.
    }
}

fn unneeded_fields(
    ir: &IntermediateRepr,
    field_type: &FieldType,
) -> Result<HashSet<String>, anyhow::Error> {
    match ir.distribute_metadata(field_type).0 {
        FieldType::Class(class_name) => {
            let class = ir
                .find_class(class_name)
                .map_err(|_| StreamingError::ExpectedClass)
                .context(format!(
                    "unneeded_fields could not look up class {class_name}",
                ))?;
            let unneeded_fields = class
                .walk_fields()
                .filter_map(|field: Walker<'_, &Field>| {
                    if field.streaming_needed() {
                        None
                    } else {
                        Some(field.name().to_string())
                    }
                })
                .collect();
            Ok(unneeded_fields)
        }
        _ => Err(StreamingError::ExpectedClass)
            .context(format!("unneeded_fields expected Class got {field_type:?}")),
    }
}

/// Whether a type must be complete before being included as a node
/// in a streamed value.
fn required_done(ir: &IntermediateRepr, field_type: &FieldType) -> bool {
    let (base_type, (_, streaming_behavior)) = ir.distribute_metadata(field_type);
    let type_implies_done = match base_type {
        FieldType::Primitive(tv) => match tv {
            TypeValue::String => false,
            TypeValue::Int => true,
            TypeValue::Float => true,
            TypeValue::Media(_) => true,
            TypeValue::Bool => true,
            TypeValue::Null => true,
        },
        FieldType::Optional(_) => false, // TODO: Think so? Or depends on Optional's base?
        FieldType::Literal(_) => true,
        FieldType::List(_) => false,
        FieldType::Map(_, _) => false,
        FieldType::Enum(_) => true,
        FieldType::Tuple(_) => false,
        FieldType::RecursiveTypeAlias(_) => false,
        FieldType::Class(_) => false,
        FieldType::Union(_) => false,
        FieldType::WithMetadata { .. } => {
            unreachable!("distribute_metadata always consumes `WithMetadata`.")
        }
    };
    let res = type_implies_done || streaming_behavior.done;
    res
}

fn completion_state(flags: &Vec<Flag>) -> CompletionState {
    if flags
        .iter()
        .any(|f| matches!(f, Flag::Incomplete) || matches!(f, Flag::Pending))
    {
        CompletionState::Incomplete
    } else {
        CompletionState::Complete
    }
}

fn type_streaming_behavior(ir: &IntermediateRepr, r#type: &FieldType) -> StreamingBehavior {
    let (_base_type, (_constraints, streaming_behavior)) = ir.distribute_metadata(r#type);
    streaming_behavior
}

#[cfg(test)]
mod tests {
    use internal_baml_core::ir::repr::make_test_ir;

    use crate::deserializer::{deserialize_flags::DeserializerConditions, types::ValueWithFlags};

    use super::*;

    fn mk_null() -> BamlValueWithFlags {
        BamlValueWithFlags::Null(DeserializerConditions::default())
    }

    fn mk_string(s: &str) -> BamlValueWithFlags {
        BamlValueWithFlags::String(ValueWithFlags {
            value: s.to_string(),
            flags: DeserializerConditions::default(),
        })
    }
    fn mk_float(s: f64) -> BamlValueWithFlags {
        BamlValueWithFlags::Float(ValueWithFlags {
            value: s,
            flags: DeserializerConditions::default(),
        })
    }

    #[test]
    fn recursive_type_alias() {
        let ir = make_test_ir(
            r##"
        type A = A[]
        "##,
        )
        .unwrap();

        fn mk_list(items: Vec<BamlValueWithFlags>) -> BamlValueWithFlags {
            BamlValueWithFlags::List(DeserializerConditions::default(), items)
        }

        let value = mk_list(vec![
            mk_list(vec![]),
            mk_list(vec![]),
            mk_list(vec![mk_list(vec![]), mk_list(vec![])]),
        ]);

        let res = validate_streaming_state(
            &ir,
            &value,
            &FieldType::RecursiveTypeAlias("A".to_string()),
            true,
        )
        .unwrap();

        assert_eq!(res.into_iter().count(), 6);
    }

    #[test]
    fn stable_keys() {
        let ir = make_test_ir(
            r##"
        class Address {
          street string
          state string
        }
        class Name {
          first string
          last string?
        }
        class Info {
          name Name
          address Address?
          hair_color string
          height float
        }
        "##,
        )
        .unwrap();

        let value = BamlValueWithFlags::Class(
            "Info".to_string(),
            DeserializerConditions::default(),
            vec![
                (
                    "name".to_string(),
                    BamlValueWithFlags::Class(
                        "Name".to_string(),
                        DeserializerConditions::default(),
                        vec![
                            ("first".to_string(), mk_string("Greg")),
                            ("last".to_string(), mk_string("Hale")),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
                ("address".to_string(), mk_null()),
                ("hair_color".to_string(), mk_string("Grey")),
                ("height".to_string(), mk_float(1.75)),
            ]
            .into_iter()
            .collect(),
        );
        let field_type = FieldType::class("Info");

        let res = validate_streaming_state(&ir, &value, &field_type, true).unwrap();


        // The first key should be "Name", matching the order specified in the
        // original value.
        match res {
            BamlValueWithMeta::Class(_name, fields, _meta) => {
                assert_eq!(fields.into_iter().next().unwrap().0.as_str(), "name");
            }
            _ => panic!("Expected Class"),
        }
    }
}
