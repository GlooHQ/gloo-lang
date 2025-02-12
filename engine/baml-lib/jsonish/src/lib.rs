pub mod helpers;
pub mod tests;

use anyhow::Result;
use indexmap::IndexMap;
pub mod deserializer;
use std::collections::HashMap;
pub mod jsonish;

use baml_types::{BamlValue, BamlValueWithMeta, FieldType, JinjaExpression, ResponseCheck};
use deserializer::{
    coercer::{ParsingContext, ParsingError, TypeCoercer},
    deserialize_flags::DeserializerConditions,
};

pub use deserializer::types::BamlValueWithFlags;
use internal_baml_core::ir::TypeValue;
use internal_baml_jinja::types::OutputFormatContent;

use crate::deserializer::score::WithScore;
use baml_types::{Completion, CompletionState};
use deserializer::deserialize_flags::Flag;
use deserializer::types::ParsingErrorToUiJson;
use jsonish::Value;
use serde::{ser::SerializeMap, ser::SerializeStruct, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct ResponseBamlValue(
    pub BamlValueWithMeta<(Vec<Flag>, Vec<ResponseCheck>, Completion)>,
);

#[derive(Debug, Clone, PartialEq)]
pub enum SerializeMode {
    Final,
    Partial,
}

/// A special-purpose wrapper for specifying the serialization format of a
/// `ResponseBamlValue`. You should construct these from `ResponseBamlValue`
/// with the `serialize_final` or `serialize_partial` method.
pub struct SerializeResponseBamlValue<'a>{
    pub value: &'a BamlValueWithMeta<(Vec<Flag>, Vec<ResponseCheck>, Completion)>,
    pub serialize_mode: SerializeMode,
}

impl ResponseBamlValue {
    /// Prepare a `ResponseBamlValue` for "final" serialization (serialization
    /// with no stream-state metadata).
    pub fn serialize_final<'a> (&'a self) -> SerializeResponseBamlValue<'a> {
        SerializeResponseBamlValue {
            value: &self.0,
            serialize_mode: SerializeMode::Final
        }
    }

    /// Prepare a `ResponseBamlValue` for "partial" serialization (serialization
    /// with stream-state metadata).
    pub fn serialize_partial<'a> (&'a self) -> SerializeResponseBamlValue<'a> {
        SerializeResponseBamlValue {
            value: &self.0,
            serialize_mode: SerializeMode::Partial
        }
    }
}

impl serde::Serialize for SerializeResponseBamlValue<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use BamlValueWithMeta::*;
        let serialize_mode = &self.serialize_mode;
        match &self.value {
            String(s, ref meta) => serialize_with_meta(&s, &meta, serialize_mode, serializer),
            Int(i, ref meta) => serialize_with_meta(&i, &meta, serialize_mode, serializer),
            Float(f, ref meta) => serialize_with_meta(&f, &meta, serialize_mode, serializer),
            Bool(b, ref meta) => serialize_with_meta(&b, &meta, serialize_mode, serializer),
            Media(v, ref meta) => serialize_with_meta(&v, &meta, serialize_mode, serializer),
            Enum(ref _name, v, ref meta) => serialize_with_meta(&v, &meta, serialize_mode, serializer),
            Map(items, ref meta) => {
                let new_items = items
                    .into_iter()
                    .map(|(k, v)| (k.clone(), SerializeResponseBamlValue{value: &v, serialize_mode: serialize_mode.clone()}))
                    .collect::<IndexMap<std::string::String, SerializeResponseBamlValue<'_>>>();
                serialize_with_meta(&new_items, &meta, serialize_mode, serializer)
            }
            List(items, ref meta) => {
                let new_items = items
                    .into_iter()
                    .map(|v| SerializeResponseBamlValue{value: v, serialize_mode: serialize_mode.clone()})
                    .collect::<Vec<_>>();
                serialize_with_meta(&new_items, &meta, serialize_mode, serializer)
            }
            Class(_name, fields, ref meta) => {
                let new_fields = fields
                    .into_iter()
                    .map(|(k, v)| {
                        let subvalue_serialize_mode = match (&serialize_mode, v.meta().2.required_done) {
                            (SerializeMode::Final, _) => SerializeMode::Final,
                            (SerializeMode::Partial, true) => SerializeMode::Final,
                            (SerializeMode::Partial, false) => SerializeMode::Partial,
                        };
                        (k, SerializeResponseBamlValue{value: v, serialize_mode: subvalue_serialize_mode})
                    })
                    .collect::<IndexMap<_, _>>();
                serialize_with_meta(&new_fields, &meta, serialize_mode, serializer)
            }
            Null(ref meta) => serialize_with_meta(&(), &meta, serialize_mode, serializer),
        }
    }
}

/// This newtype wrapper exists solely for the purpose of defining a
/// `Serialize` impl.
pub struct ResponseChecksMetadata<'a, T: Serialize>(pub (&'a T, &'a Vec<ResponseCheck>));

impl<'a, T: Serialize> serde::Serialize for ResponseChecksMetadata<'a, T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let checks_map: HashMap<_, _> = self
            .0
             .1
            .iter()
            .map(|check| (check.name.clone(), check))
            .collect();
        let mut state = serializer.serialize_struct("Checked", 2)?;
        state.serialize_field("value", &self.0 .0)?;
        state.serialize_field("checks", &checks_map)?;
        state.end()
    }
}

fn serialize_with_meta<S: Serializer, T: Serialize>(
    value: &T,
    meta: &(Vec<Flag>, Vec<ResponseCheck>, Completion),
    serialize_mode: &SerializeMode,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let should_display_stream_state = meta.2.display && matches!(serialize_mode, SerializeMode::Partial);
    match (meta.1.len(), should_display_stream_state) {
        (0, false) => value.serialize(serializer),
        (_, false) => ResponseChecksMetadata((value, &meta.1)).serialize(serializer),
        (0, true) => {
            let mut state = serializer.serialize_struct("StreamState", 2)?;
            state.serialize_field("state", &meta.2.state)?;
            state.serialize_field("value", value)?;
            state.end()
        }
        (_, true) => {
            let mut outer_value = serializer.serialize_struct("StreamState", 2)?;
            outer_value.serialize_field("state", &meta.2.state)?;
            outer_value.serialize_field("value", &ResponseChecksMetadata((value, &meta.1)))?;
            outer_value.end()
        }
    }
}

pub fn from_str(
    of: &OutputFormatContent,
    target: &FieldType,
    raw_string: &str,
    allow_partials: bool,
) -> Result<BamlValueWithFlags> {
    if matches!(target, FieldType::Primitive(TypeValue::String)) {
        return Ok(BamlValueWithFlags::String(raw_string.to_string().into()));
    }

    // When the schema is just a string, i should really just return the raw_string w/o parsing it.
    let value = jsonish::parse(raw_string, jsonish::ParseOptions::default())?;

    // Pick the schema that is the most specific.
    log::debug!("Parsed JSONish (step 1 of parsing): {:#?}", value);
    let ctx = ParsingContext::new(of, allow_partials);

    // Determine the best way to get the desired schema from the parsed schema.

    // Lets try to now coerce the value into the expected schema.
    let parsed_value: BamlValueWithFlags = match target.coerce(&ctx, target, Some(&value)) {
        Ok(v) => {
            if v.conditions()
                .flags()
                .iter()
                .any(|f| matches!(f, Flag::InferedObject(jsonish::Value::String(_, _))))
            {
                anyhow::bail!("Failed to coerce value: {:?}", v.conditions().flags());
            }

            Ok::<BamlValueWithFlags, anyhow::Error>(v)
        }
        Err(e) => anyhow::bail!("Failed to coerce value: {}", e),
    }?;

    Ok(parsed_value)
}

impl ResponseBamlValue {
    pub fn score(&self) -> i32 {
        self.0.iter().map(|node| node.meta().0.score()).sum()
    }

    pub fn explanation_json(&self) -> Vec<serde_json::Value> {
        let mut expl = vec![];
        self.explanation_impl(vec!["<root>".to_string()], &mut expl);
        expl.into_iter().map(|e| e.to_ui_json()).collect::<Vec<_>>()
    }

    fn explanation_impl(&self, scope: Vec<String>, expls: &mut Vec<ParsingError>) {
        self.0.iter().for_each(|node| {
            let message = match node {
                BamlValueWithMeta::String(_, _) => "error while parsing string".to_string(),
                BamlValueWithMeta::Int(_, _) => "error while parsing int".to_string(),
                BamlValueWithMeta::Float(_, _) => "error while parsing float".to_string(),
                BamlValueWithMeta::Bool(_, _) => "error while parsing bool".to_string(),
                BamlValueWithMeta::List(_, _) => "error while parsing list".to_string(),
                BamlValueWithMeta::Map(_, _) => "error while parsing map".to_string(),
                BamlValueWithMeta::Enum(enum_name, _, _) => {
                    format!("error while parsing {enum_name} enum value")
                }
                BamlValueWithMeta::Class(class_name, _, _) => {
                    format!("error while parsing class {class_name}")
                }
                BamlValueWithMeta::Null(_) => "error while parsing null".to_string(),
                BamlValueWithMeta::Media(_, _) => "error while parsing media".to_string(),
            };
            let parsing_error = ParsingError {
                scope: scope.clone(),
                reason: message,
                causes: DeserializerConditions {
                    flags: node.meta().0.clone(),
                }
                .explanation(),
            };
            if node.meta().0.len() > 0 {
                expls.push(parsing_error)
            }
        })
    }
}

impl From<ResponseBamlValue> for BamlValue {
    fn from(v: ResponseBamlValue) -> BamlValue {
        v.0.into()
    }
}

impl WithScore for ResponseBamlValue {
    fn score(&self) -> i32 {
        self.0.iter().map(|node| node.meta().0.score()).sum()
    }
}