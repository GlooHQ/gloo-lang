use std::collections::HashSet;

use baml_types::{BamlMap, BamlMedia, BamlValue, BamlValueWithMeta, Constraint, JinjaExpression};
use serde_json::json;
use strsim::jaro;

use super::{
    coercer::ParsingError,
    deserialize_flags::{DeserializerConditions, Flag},
    score::WithScore,
};

// Recursive parity
#[derive(Clone, Debug)]
pub enum BamlValueWithFlags {
    String(ValueWithFlags<String>),
    Int(ValueWithFlags<i64>),
    Float(ValueWithFlags<f64>),
    Bool(ValueWithFlags<bool>),
    List(DeserializerConditions, Vec<BamlValueWithFlags>),
    Map(
        DeserializerConditions,
        BamlMap<String, (DeserializerConditions, BamlValueWithFlags)>,
    ),
    Enum(String, ValueWithFlags<String>),
    Class(
        String,
        DeserializerConditions,
        BamlMap<String, BamlValueWithFlags>,
    ),
    Null(DeserializerConditions),
    Media(ValueWithFlags<BamlMedia>),
}

impl BamlValueWithFlags {
    pub fn is_composite(&self) -> bool {
        match self {
            BamlValueWithFlags::String(_)
            | BamlValueWithFlags::Int(_)
            | BamlValueWithFlags::Float(_)
            | BamlValueWithFlags::Bool(_)
            | BamlValueWithFlags::Null(_)
            | BamlValueWithFlags::Enum(_, _) => false,

            BamlValueWithFlags::List(_, _)
            | BamlValueWithFlags::Map(_, _)
            | BamlValueWithFlags::Class(_, _, _)
            | BamlValueWithFlags::Media(_) => true,
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            BamlValueWithFlags::String(f) => f.score(),
            BamlValueWithFlags::Int(f) => f.score(),
            BamlValueWithFlags::Float(f) => f.score(),
            BamlValueWithFlags::Bool(f) => f.score(),
            BamlValueWithFlags::List(f, items) => {
                f.score() + items.iter().map(|i| i.score()).sum::<i32>()
            }
            BamlValueWithFlags::Map(f, kv) => {
                f.score()
                    + kv.iter()
                        .map(|(_, (f, v))| f.score() + v.score())
                        .sum::<i32>()
            }
            BamlValueWithFlags::Enum(_, f) => f.score(),
            BamlValueWithFlags::Class(_, f, items) => {
                f.score() + items.iter().map(|(_, v)| v.score()).sum::<i32>()
            }
            BamlValueWithFlags::Null(f) => f.score(),
            BamlValueWithFlags::Media(f) => f.score(),
        }
    }

    pub fn conditions(&self) -> &DeserializerConditions {
        match self {
            BamlValueWithFlags::String(v) => &v.flags,
            BamlValueWithFlags::Int(v) => &v.flags,
            BamlValueWithFlags::Float(v) => &v.flags,
            BamlValueWithFlags::Bool(v) => &v.flags,
            BamlValueWithFlags::List(v, _) => v,
            BamlValueWithFlags::Map(v, _) => v,
            BamlValueWithFlags::Enum(_, v) => &v.flags,
            BamlValueWithFlags::Class(_, v, _) => v,
            BamlValueWithFlags::Null(v) => v,
            BamlValueWithFlags::Media(v) => &v.flags,
        }
    }
}

impl From<BamlValueWithFlags> for BamlValueWithMeta<Vec<Flag>> {
    fn from(baml_value: BamlValueWithFlags) -> BamlValueWithMeta<Vec<Flag>> {
        match baml_value {
            BamlValueWithFlags::String(v) => BamlValueWithMeta::String(v.value, v.flags.flags),
            BamlValueWithFlags::Int(v) => BamlValueWithMeta::Int(v.value, v.flags.flags),
            BamlValueWithFlags::Float(v) => BamlValueWithMeta::Float(v.value, v.flags.flags),
            BamlValueWithFlags::Bool(v) => BamlValueWithMeta::Bool(v.value, v.flags.flags),
            BamlValueWithFlags::List(conditions, items) => {
                BamlValueWithMeta::List(items.into_iter().map(|v| BamlValueWithMeta::from(v)).collect(), conditions.flags)
            },
            BamlValueWithFlags::Map(conditions, fields) => BamlValueWithMeta::Map(
                 // NOTE: For some reason, Map is map<key, (conds, v)>, even though `v` contains conds.
                 // Maybe the extra conds are for the field, not the value?
                fields.into_iter().map(|(k,v)| (k, BamlValueWithMeta::from(v.1))).collect(), conditions.flags
            ),
            BamlValueWithFlags::Enum(n,v) => BamlValueWithMeta::Enum(n, v.value, v.flags.flags),
            BamlValueWithFlags::Class(name, conds, fields) =>
                BamlValueWithMeta::Class(name, fields.into_iter().map(|(k,v)| (k, BamlValueWithMeta::from(v))).collect(), conds.flags),
            BamlValueWithFlags::Null(v) => BamlValueWithMeta::Null(v.flags),
            BamlValueWithFlags::Media(v) => BamlValueWithMeta::Media(v.value, v.flags.flags),
        }
    }
}

pub trait ParsingErrorToUiJson {
    fn to_ui_json(&self) -> serde_json::Value;
}

impl ParsingErrorToUiJson for ParsingError {
    fn to_ui_json(&self) -> serde_json::Value {
        json!({
            if self.scope.is_empty() {
                "<root>".to_string()
            } else {
                self.scope.join(".")
            }: self.reason,
            "causes": self.causes.iter().map(|c| c.to_ui_json()).collect::<Vec<_>>(),
        })
    }
}

impl BamlValueWithFlags {
    pub fn explanation_json(&self) -> Vec<serde_json::Value> {
        let mut expl = vec![];
        self.explanation_impl(vec!["<root>".to_string()], &mut expl);
        expl.into_iter().map(|e| e.to_ui_json()).collect::<Vec<_>>()
    }

    pub fn explanation_impl(&self, scope: Vec<String>, expls: &mut Vec<ParsingError>) {
        match self {
            BamlValueWithFlags::String(v) => {
                let causes = v.flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing string".to_string(),
                        causes,
                    });
                }
            }
            BamlValueWithFlags::Int(v) => {
                let causes = v.flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing int".to_string(),
                        causes,
                    });
                }
            }
            BamlValueWithFlags::Float(v) => {
                let causes = v.flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing float".to_string(),
                        causes,
                    });
                }
            }
            BamlValueWithFlags::Bool(v) => {
                let causes = v.flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing bool".to_string(),
                        causes,
                    });
                }
            }
            BamlValueWithFlags::List(flags, values) => {
                let causes = flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing list".to_string(),
                        causes,
                    });
                }
                for (i, value) in values.iter().enumerate() {
                    let mut scope = scope.clone();
                    scope.push(format!("parsed:{}", i));
                    value.explanation_impl(scope, expls);
                }
            }
            BamlValueWithFlags::Map(flags, kv) => {
                let causes = flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing map".to_string(),
                        causes,
                    });
                }
                for (k, (v_flags, v)) in kv.iter() {
                    let causes = v_flags.explanation();
                    if !causes.is_empty() {
                        expls.push(ParsingError {
                            scope: scope.clone(),
                            reason: format!("error while parsing value for map key '{}'", k),
                            causes,
                        });
                    }
                    let mut scope = scope.clone();
                    scope.push(format!("parsed:{}", k));
                    v.explanation_impl(scope, expls);
                }
            }
            BamlValueWithFlags::Enum(enum_name, v) => {
                let causes = v.flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: format!("error while parsing {enum_name} enum value"),
                        causes,
                    });
                }
            }
            BamlValueWithFlags::Class(class_name, v, fields) => {
                let causes = v.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: format!("error while parsing class {}", class_name),
                        causes,
                    });
                }
                for (k, v) in fields.iter() {
                    let mut scope = scope.clone();
                    scope.push(k.to_string());
                    v.explanation_impl(scope, expls);
                }
            }

            BamlValueWithFlags::Null(v) => {
                let causes = v.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing null".to_string(),
                        causes,
                    });
                }
            }
            BamlValueWithFlags::Media(v) => {
                let causes = v.flags.explanation();
                if !causes.is_empty() {
                    expls.push(ParsingError {
                        scope: scope.clone(),
                        reason: "error while parsing media".to_string(),
                        causes,
                    });
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueWithFlags<T> {
    pub value: T,
    pub flags: DeserializerConditions,
}

impl<T> ValueWithFlags<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> From<T> for ValueWithFlags<T> {
    fn from(item: T) -> Self {
        ValueWithFlags {
            value: item,
            flags: DeserializerConditions::new(),
        }
    }
}

impl<T> From<(T, &[Flag])> for ValueWithFlags<T> {
    fn from((value, flags): (T, &[Flag])) -> Self {
        let flags = flags
            .iter()
            .fold(DeserializerConditions::new(), |acc, flag| {
                acc.with_flag(flag.to_owned())
            });
        ValueWithFlags { value, flags }
    }
}

impl<T> From<(T, Flag)> for ValueWithFlags<T> {
    fn from((value, flag): (T, Flag)) -> Self {
        ValueWithFlags {
            value,
            flags: DeserializerConditions::new().with_flag(flag),
        }
    }
}

impl<T> From<(T, DeserializerConditions)> for ValueWithFlags<T> {
    fn from((value, flags): (T, DeserializerConditions)) -> Self {
        ValueWithFlags { value, flags }
    }
}

impl From<BamlValueWithFlags> for BamlValue {
    fn from(value: BamlValueWithFlags) -> BamlValue {
        match value {
            BamlValueWithFlags::String(s) => BamlValue::String(s.value),
            BamlValueWithFlags::Int(i) => BamlValue::Int(i.value),
            BamlValueWithFlags::Float(f) => BamlValue::Float(f.value),
            BamlValueWithFlags::Bool(b) => BamlValue::Bool(b.value),
            BamlValueWithFlags::List(_, v) => {
                BamlValue::List(v.into_iter().map(|x| x.into()).collect())
            }
            BamlValueWithFlags::Map(_, m) => {
                BamlValue::Map(m.into_iter().map(|(k, (_, v))| (k, v.into())).collect())
            }
            BamlValueWithFlags::Enum(s, v) => BamlValue::Enum(s, v.value),
            BamlValueWithFlags::Class(s, _, m) => {
                BamlValue::Class(s, m.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            BamlValueWithFlags::Null(_) => BamlValue::Null,
            BamlValueWithFlags::Media(i) => BamlValue::Media(i.value),
        }
    }
}

impl From<&BamlValueWithFlags> for BamlValue {
    fn from(value: &BamlValueWithFlags) -> BamlValue {
        match value {
            BamlValueWithFlags::String(s) => BamlValue::String(s.value.clone()),
            BamlValueWithFlags::Int(i) => BamlValue::Int(i.value),
            BamlValueWithFlags::Float(f) => BamlValue::Float(f.value),
            BamlValueWithFlags::Bool(b) => BamlValue::Bool(b.value),
            BamlValueWithFlags::List(_, v) => BamlValue::List(v.iter().map(|x| x.into()).collect()),
            BamlValueWithFlags::Map(_, m) => BamlValue::Map(
                m.into_iter()
                    .map(|(k, (_, v))| (k.clone(), v.into()))
                    .collect(),
            ),
            BamlValueWithFlags::Enum(s, v) => BamlValue::Enum(s.clone(), v.value.clone()),
            BamlValueWithFlags::Class(s, _, m) => BamlValue::Class(
                s.clone(),
                m.into_iter().map(|(k, v)| (k.clone(), v.into())).collect(),
            ),
            BamlValueWithFlags::Null(_) => BamlValue::Null,
            BamlValueWithFlags::Media(i) => BamlValue::Media(i.value.clone()),
        }
    }
}

impl BamlValueWithFlags {
    pub(super) fn add_flag(&mut self, flag: Flag) {
        match self {
            BamlValueWithFlags::String(v) => v.flags.add_flag(flag),
            BamlValueWithFlags::Int(v) => v.flags.add_flag(flag),
            BamlValueWithFlags::Float(v) => v.flags.add_flag(flag),
            BamlValueWithFlags::Bool(v) => v.flags.add_flag(flag),
            BamlValueWithFlags::List(v, _) => v.add_flag(flag),
            BamlValueWithFlags::Map(v, _) => v.add_flag(flag),
            BamlValueWithFlags::Enum(_, v) => v.flags.add_flag(flag),
            BamlValueWithFlags::Class(_, v, _) => v.add_flag(flag),
            BamlValueWithFlags::Null(v) => v.add_flag(flag),
            BamlValueWithFlags::Media(v) => v.flags.add_flag(flag),
        }
    }

    pub(super) fn r#type(&self) -> String {
        match self {
            BamlValueWithFlags::String(_) => "String".to_string(),
            BamlValueWithFlags::Int(_) => "Int".to_string(),
            BamlValueWithFlags::Float(_) => "Float".to_string(),
            BamlValueWithFlags::Bool(_) => "Bool".to_string(),
            BamlValueWithFlags::List(_, i) => {
                let inner = i
                    .iter()
                    .map(|i| i.r#type())
                    .collect::<HashSet<String>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(" | ");
                format!("List[{}:{inner}]", i.len())
            }
            BamlValueWithFlags::Map(_, _) => "Map".to_string(),
            BamlValueWithFlags::Enum(n, _) => format!("Enum {n}"),
            BamlValueWithFlags::Class(c, _, _) => format!("Class {c}"),
            BamlValueWithFlags::Null(_) => "Null".to_string(),
            BamlValueWithFlags::Media(_) => "Image".to_string(),
        }
    }
}

impl std::fmt::Display for BamlValueWithFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Score: {}): ", self.r#type(), self.score())?;
        match self {
            BamlValueWithFlags::String(v) => {
                write!(f, "{}", v.value)?;
                if !v.flags.flags.is_empty() {
                    write!(f, "\n  {}", v.flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::Int(v) => {
                write!(f, "{}", v.value)?;
                if !v.flags.flags.is_empty() {
                    write!(f, "\n  {}", v.flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::Float(v) => {
                write!(f, "{}", v.value)?;
                if !v.flags.flags.is_empty() {
                    write!(f, "\n  {}", v.flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::Bool(v) => {
                write!(f, "{}", v.value)?;
                if !v.flags.flags.is_empty() {
                    write!(f, "\n  {}", v.flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::List(flags, v) => {
                writeln!(f)?;
                for (idx, item) in v.iter().enumerate() {
                    writeln!(f, "  {idx}: {}", item.to_string().replace("\n", "  \n"))?;
                }
                if !flags.flags.is_empty() {
                    writeln!(f, "  {}", flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::Map(_, v) => {
                writeln!(f)?;
                for (key, value) in v {
                    writeln!(f, "{}: {}", key, value.1)?;
                }
            }
            BamlValueWithFlags::Enum(_n, v) => {
                write!(f, "{}", v.value)?;
                if !v.flags.flags.is_empty() {
                    write!(f, "\n  {}", v.flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::Class(_, flags, v) => {
                writeln!(f)?;
                for (k, v) in v.iter() {
                    writeln!(f, "  KV {}", k.to_string().replace("\n", "\n  "))?;
                    writeln!(f, "  {}", v.to_string().replace("\n", "\n  "))?;
                }
                if !flags.flags.is_empty() {
                    writeln!(f, "  {}", flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::Null(flags) => {
                write!(f, "null")?;
                if !flags.flags.is_empty() {
                    write!(f, "\n  {}", flags.to_string().replace("\n", "\n  "))?;
                }
            }
            BamlValueWithFlags::Media(v) => {
                write!(f, "{:#?}", v.value)?;
                if !v.flags.flags.is_empty() {
                    write!(f, "\n  {}", v.flags.to_string().replace("\n", "\n  "))?;
                }
            }
        };

        Ok(())
    }
}

impl From<BamlValueWithFlags> for BamlValueWithMeta<Vec<(String, JinjaExpression, bool)>> {
    fn from(baml_value: BamlValueWithFlags) -> Self {
        use BamlValueWithFlags::*;
        let c = baml_value.conditions().constraint_results();
        match baml_value {
            String(ValueWithFlags { value, .. }) => BamlValueWithMeta::String(value, c),
            Int(ValueWithFlags { value, .. }) => BamlValueWithMeta::Int(value, c),
            Float(ValueWithFlags { value, .. }) => BamlValueWithMeta::Float(value, c),
            Bool(ValueWithFlags { value, .. }) => BamlValueWithMeta::Bool(value, c),
            Map(_, values) => BamlValueWithMeta::Map(
                values.into_iter().map(|(k, v)| (k, v.1.into())).collect(),
                c,
            ), // TODO: (Greg) I discard the DeserializerConditions tupled up with the value of the BamlMap. I'm not sure why BamlMap value is (DeserializerContitions, BamlValueWithFlags) in the first place.
            List(_, values) => {
                BamlValueWithMeta::List(values.into_iter().map(|v| v.into()).collect(), c)
            }
            Media(ValueWithFlags { value, .. }) => BamlValueWithMeta::Media(value, c),
            Enum(enum_name, ValueWithFlags { value, .. }) => {
                BamlValueWithMeta::Enum(enum_name, value, c)
            }
            Class(class_name, _, fields) => BamlValueWithMeta::Class(
                class_name,
                fields.into_iter().map(|(k, v)| (k, v.into())).collect(),
                c,
            ),
            Null(_) => BamlValueWithMeta::Null(c),
        }
    }
}
