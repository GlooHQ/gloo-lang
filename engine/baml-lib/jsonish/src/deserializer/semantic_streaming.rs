// This module helps resolve baml values with attached streaming state
// in the context of the streaming behavior associated with their types.


use crate::deserializer::coercer::ParsingError;
use crate::{BamlValueWithFlags, Flag};
use indexmap::IndexMap;
use internal_baml_core::ir::repr::{IntermediateRepr, Walker};
use internal_baml_core::ir::{Field, IRHelper};

use baml_types::{BamlValueWithMeta, CompletionState, FieldType, ResponseCheck, StreamingBehavior, TypeValue};

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub enum StreamingError {
    ExpectedClass,
    IncompleteDoneValue,
    MissingNeededFields,
}

/// For a given baml value, traverse its nodes, comparing the completion state
/// of each node against the streaming behavior of the node's type.
pub fn validate_streaming_state(
    ir: &IntermediateRepr,
    baml_value: &BamlValueWithFlags,
    field_type: &FieldType,
    allow_partials: bool,
) -> Result<BamlValueWithMeta<Option<CompletionState>>, StreamingError> {
    let baml_value_with_meta_flags: BamlValueWithMeta<Vec<Flag>> = baml_value.clone().into();
    let typed_baml_value: BamlValueWithMeta<(Vec<Flag>, FieldType)> = ir
        .distribute_type_with_meta(baml_value_with_meta_flags, field_type.clone())
        .unwrap();
    let baml_value_with_streaming_state_and_behavior =
        typed_baml_value.map_meta(|(flags, r#type)| (completion_state(&flags), r#type));

    process_node(ir, baml_value_with_streaming_state_and_behavior, allow_partials)
}

/// Consider a node's type, streaming state, and streaming behavior annotations. Return
/// an error if streaming state doesn't meet the streaming requirements. Also attach
/// the streaming state to the node as metadata, if this was requested by the user
/// vial `@stream.with_state`.
///
/// This function descends into child nodes, when the argument is a compound value.
fn process_node(
    ir: &IntermediateRepr,
    value: BamlValueWithMeta<(CompletionState, &FieldType)>,
    allow_partials: bool,
) -> Result<BamlValueWithMeta<Option<CompletionState>>, StreamingError> {
    let (completion_state, field_type) = value.meta();
    let (base_type, (_, streaming_behavior)) = ir.distribute_metadata(field_type);

    let must_be_done = required_done(ir, field_type) && allow_partials;

    // eprintln!("Working on {value:?}");
    // eprintln!("  completion: {completion_state:?}");
    // eprintln!("  field_type: {field_type:?}");
    // eprintln!("  mustbedone: {}", must_be_done);

    if must_be_done && !(completion_state == &CompletionState::Complete) {
        // eprintln!("  Aborting because incomplete");
        return Err(StreamingError::IncompleteDoneValue);
    }

    let new_meta = if streaming_behavior.state {
        Some(completion_state.clone())
    } else {
        None
    };
    // eprintln!("  new_meta: {:?}", new_meta);

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
                .filter_map(|item| process_node(ir, item, allow_partials).ok())
                .collect(),
            new_meta,
        )),
        BamlValueWithMeta::Class(ref class_name, ref fields, _) => {
            let needed_fields: HashSet<String> = needed_fields(ir, field_type, allow_partials)?;
            let mut new_fields = fields
                .clone()
                .into_iter()
                .filter_map(|(field_name, field_value)| process_node(ir, field_value, allow_partials).ok().map(|v| (field_name, v)))
                .collect::<IndexMap<String,BamlValueWithMeta<_>>>();
            let new_field_names = new_fields.iter().filter_map(|(field_name, field_value)|
                match field_value {
                    BamlValueWithMeta::Null(_) => None,
                    _ => Some(field_name.clone()),
                }
            ).collect();
            let missing_needed_fields = needed_fields.difference(&new_field_names);
            let nulls_for_unneeded_fields = fields.iter().filter_map(|(field_name, field)| {
                if needed_fields.contains(field_name) || new_fields.get(field_name).is_some() {
                    None
                } else {
                    let completion_state = field.meta().1.streaming_behavior().state;
                    let field_stream_state = if completion_state { Some(CompletionState::Incomplete) } else { None };
                    Some((field_name.clone(), BamlValueWithMeta::Null(field_stream_state)))
                }
            }).collect::<IndexMap<String, BamlValueWithMeta<_>>>();
            dbg!(&nulls_for_unneeded_fields);

            new_fields.extend(nulls_for_unneeded_fields);
            dbg!(&new_fields);
            if missing_needed_fields.clone().count() == 0 {
                Ok(BamlValueWithMeta::Class(class_name.clone(), new_fields, new_meta))
            } else {
                // eprintln!("  Missing needed fields: {missing_needed_fields:?}");
                Err(StreamingError::MissingNeededFields)
            }

        }
        BamlValueWithMeta::Enum(name, value, _) => Ok(BamlValueWithMeta::Enum(name, value, new_meta)),
        BamlValueWithMeta::Map(kvs, _) => {
            let new_kvs = kvs.into_iter().filter_map(|(k,v)| process_node(ir, v, allow_partials).ok().map(|v| (k,v))).collect();
            Ok(BamlValueWithMeta::Map(new_kvs, new_meta))
        }
    };

    // eprintln!("  new_value: {new_value:?}");
    // let mut value_meta = new_value.meta_mut();
    // *value_meta = new_meta;
    new_value
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
) -> Result<HashSet<String>, StreamingError> {
    if allow_partials == false {
        return Ok(HashSet::new());
    }
    match field_type {
        FieldType::Class(class_name) => {
            let class = ir
                .find_class(class_name)
                .map_err(|_| StreamingError::ExpectedClass)?;
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
        _ => Err(StreamingError::ExpectedClass), // TODO: Handle type aliases?.
    }
}

fn unneeded_fields(
    ir: &IntermediateRepr,
    field_type: &FieldType
) -> Result<HashSet<String>, StreamingError> {
    match field_type {
        FieldType::Class(class_name) => {
            let class = ir.find_class(class_name).map_err(|_| StreamingError::ExpectedClass)?;
            let unneeded_fields = class
                .walk_fields()
                .filter_map(|field: Walker<'_, &Field>| {
                if field.streaming_needed() {
                    None
                } else {
                    Some(field.name().to_string())
                }
            }).collect();
            Ok(unneeded_fields)
        },
        _ => Err(StreamingError::ExpectedClass),
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
        FieldType::Class(_) => false,
        FieldType::Union(_) => false,
        FieldType::WithMetadata { .. } => {
            unreachable!("distribute_metadata always consumes `WithMetadata`.")
        }
    };
    // eprintln!("  type_implies_done: {type_implies_done:?}");
    // eprintln!("  streaming_has_done: {:?}", streaming_behavior.done);
    let res = type_implies_done || streaming_behavior.done;
    // eprintln!("  res: {:?}", res);
    res
}

fn completion_state(flags: &Vec<Flag>) -> CompletionState {
    if flags.iter().any(|f| matches!(f, Flag::Incomplete)) {
        CompletionState::Incomplete
    } else {
        CompletionState::Complete
    }
}

fn streaming_behavior(ir: &IntermediateRepr, r#type: &FieldType) -> StreamingBehavior {
    let (_base_type, (_constraints, streaming_behavior)) = ir.distribute_metadata(r#type);
    streaming_behavior
}
