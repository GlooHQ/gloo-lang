use core::panic;
use std::{collections::HashMap};

use anyhow::Result;
use opentelemetry::propagation::Injector;
use tracing::{field::Visit};

use crate::{
    api_wrapper::core_types::{IOValue, TypeSchema, TypeSchemaName, ValueType},
    baml_event_def,
};

use super::partial_types::{Apply, PartialLogSchema};

#[derive(Default)]

pub(crate) struct IOEvent {
    is_input: bool,
    args: FunctionArgs,
}

pub type ArgAndType = (String, String);
pub type NamedArgs = HashMap<String, ArgAndType>;
pub type PositionalArgs = Vec<(Option<String>, ArgAndType)>;
pub type FunctionArgs = (PositionalArgs, NamedArgs);

impl IOEvent {
    pub fn input_event(args: &FunctionArgs) -> Result<()> {
        let args = serde_json::to_string(args)?;
        let is_input = true;
        baml_event_def!(InputOutput, is_input, args);
        Ok(())
    }
    pub fn output_event(value: &String, r#type: &String) -> Result<()> {
        let packet: PositionalArgs = vec![(Some("result".into()), (value.clone(), r#type.clone()))];
        let args: FunctionArgs = (packet, Default::default());
        let args = serde_json::to_string(&args)?;
        let is_input = false;
        baml_event_def!(InputOutput, is_input, args);
        Ok(())
    }
}

impl Visit for IOEvent {
    fn record_debug(&mut self, field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {
        panic!("unexpected field name: {}", field.name());
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        match field.name() {
            "is_input" => self.is_input = value,
            name => {
                panic!("unexpected field name: {}", name);
            }
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "args" => {
                self.args = serde_json::from_str(value).unwrap_or_default();
            }
            name => {
                panic!("unexpected field name: {}", name);
            }
        }
    }
}

impl From<IOEvent> for Option<IOValue> {
    fn from(value: IOEvent) -> Self {
        let (positional, named) = &value.args;

        match (positional.is_empty(), named.is_empty()) {
            (true, true) => None,
            (false, true) => match positional.len() {
                1 => {
                    let (name, (value, r#type)) = positional.first().unwrap();
                    Some(IOValue {
                        r#type: TypeSchema {
                            name: TypeSchemaName::Single,
                            fields: [(
                                name.as_ref().map(|n| n.into()).unwrap_or("arg0".into()),
                                r#type.clone(),
                            )]
                            .into(),
                        },
                        value: ValueType::String(value.clone()),
                        r#override: None,
                    })
                }
                _ => {
                    let mut values = Vec::default();
                    let mut types = HashMap::default();
                    for (pos, (name, (value, r#type))) in positional.iter().enumerate() {
                        values.push(value.clone());
                        types.set(
                            name.as_deref().unwrap_or(format!("arg{pos}").as_str()),
                            r#type.clone(),
                        );
                    }
                    Some(IOValue {
                        r#type: TypeSchema {
                            name: TypeSchemaName::Multi,
                            fields: types,
                        },
                        value: ValueType::List(values),
                        r#override: None,
                    })
                }
            },
            (true, false) => {
                let mut values = Vec::default();
                let mut types = HashMap::default();
                for (name, (value, r#type)) in named {
                    values.push(value.clone());
                    types.set(name.as_str(), r#type.clone());
                }
                Some(IOValue {
                    r#type: TypeSchema {
                        name: TypeSchemaName::Multi,
                        fields: types,
                    },
                    value: ValueType::List(values),
                    r#override: None,
                })
            }
            (false, false) => {
                panic!("Cannot have both positional and named arguments")
            }
        }

        // match value.values.len() {
        //     0 => None,
        //     1 => Some(IOValue {
        //         r#type: TypeSchema {
        //             name: TypeSchemaName::Single,
        //             fields: mem::take(&mut value.types),
        //         },
        //         value: ValueType::String(value.values.pop().unwrap()),
        //         r#override: None,
        //     }),
        //     _ => Some(IOValue {
        //         r#type: TypeSchema {
        //             name: TypeSchemaName::Multi,
        //             fields: mem::take(&mut value.types),
        //         },
        //         value: ValueType::List(mem::take(&mut value.values)),
        //         r#override: None,
        //     }),
        // }
    }
}

impl<'a, S> Apply<'a, IOEvent, S> for PartialLogSchema
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn apply(&mut self, event: IOEvent, _span: &tracing_subscriber::registry::SpanRef<'a, S>) {
        match event.is_input {
            true => self.io.input = event.into(),
            false => self.io.output = event.into(),
        }
    }
}
