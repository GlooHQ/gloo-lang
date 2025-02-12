use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use anyhow::{Context,Result};
use indexmap::IndexMap;
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

use crate::{media::BamlMediaType, ResponseCheck};
use crate::{BamlMap, BamlMedia};

#[derive(Clone, Debug, PartialEq)]
pub enum BamlValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Map(BamlMap<String, BamlValue>),
    List(Vec<BamlValue>),
    Media(BamlMedia),
    Enum(String, String),
    Class(String, BamlMap<String, BamlValue>),
    Null,
}

impl serde::Serialize for BamlValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            BamlValue::String(s) => serializer.serialize_str(s),
            BamlValue::Int(i) => serializer.serialize_i64(*i),
            BamlValue::Float(f) => serializer.serialize_f64(*f),
            BamlValue::Bool(b) => serializer.serialize_bool(*b),
            BamlValue::Map(m) => m.serialize(serializer),
            BamlValue::List(l) => l.serialize(serializer),
            BamlValue::Media(m) => {
                m.serialize(serializer)
            }
            BamlValue::Enum(_, v) => serializer.serialize_str(v),
            BamlValue::Class(_, m) => m.serialize(serializer),
            BamlValue::Null => serializer.serialize_none(),
        }
    }
}

impl BamlValue {
    pub fn r#type(&self) -> String {
        match self {
            BamlValue::String(_) => "string".into(),
            BamlValue::Int(_) => "int".into(),
            BamlValue::Float(_) => "float".into(),
            BamlValue::Bool(_) => "bool".into(),
            BamlValue::Map(kv) => {
                let value_types = kv
                    .values()
                    .map(|v| v.r#type())
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(" | ");
                if value_types.is_empty() {
                    "map<string, ?>".into()
                } else {
                    format!("map<string, {value_types}>")
                }
            }
            BamlValue::List(k) => {
                let value_type = k
                    .iter()
                    .map(|v| v.r#type())
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(" | ");
                if value_type.is_empty() {
                    "list<?>".into()
                } else {
                    format!("list<{value_type}>")
                }
            }
            BamlValue::Media(m) => match m.media_type {
                BamlMediaType::Image => "image",
                BamlMediaType::Audio => "audio",
            }
            .into(),
            BamlValue::Enum(e, _) => format!("enum {e}"),
            BamlValue::Class(c, _) => format!("class {c}"),
            BamlValue::Null => "null".into(),
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            BamlValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            BamlValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            BamlValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_map(&self) -> bool {
        matches!(self, BamlValue::Map(_))
    }

    pub fn as_map(&self) -> Option<&BamlMap<String, BamlValue>> {
        match self {
            BamlValue::Map(m) => Some(m),
            _ => None,
        }
    }
    pub fn as_map_owned(self) -> Option<BamlMap<String, BamlValue>> {
        match self {
            BamlValue::Map(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_list_owned(self) -> Option<Vec<BamlValue>> {
        match self {
            BamlValue::List(vals) => Some(vals),
            _ => None,
        }
    }
}

impl std::fmt::Display for BamlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", serde_json::json!(self))
    }
}

impl<'de> Deserialize<'de> for BamlValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BamlValueVisitor)
    }
}

struct BamlValueVisitor;

impl<'de> Visitor<'de> for BamlValueVisitor {
    type Value = BamlValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid BamlValue")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::String(value.to_owned()))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(v as i64))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(v as i64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(v as i64))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(v as i64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(v as i64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(v as i64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(v as i64))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Int(value))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Float(v as f64))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Float(value))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::String(v))
    }

    fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::custom("bytes are not supported by BAML"))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::String(v.to_owned()))
    }

    fn visit_i128<E>(self, _: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::custom("i128 is not supported by BAML"))
    }

    fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::custom("u128 is not supported by BAML"))
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Bool(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        BamlValue::deserialize(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BamlValue::Null)
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(BamlValue::List(values))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut values = BamlMap::new();
        while let Some((key, value)) = map.next_entry()? {
            values.insert(key, value);
        }
        Ok(BamlValue::Map(values))
    }
}

/// A BamlValue with associated metadata.
/// This type is used to flexibly carry additional information.
/// It is used as a base type for situations where we want to represent
/// a BamlValue with additional information per node, such as a score,
/// or a constraint result.
#[derive(Clone, Debug, PartialEq)]
pub enum BamlValueWithMeta<T> {
    String(String, T),
    Int(i64, T),
    Float(f64, T),
    Bool(bool, T),
    Map(BamlMap<String, BamlValueWithMeta<T>>, T),
    List(Vec<BamlValueWithMeta<T>>, T),
    Media(BamlMedia, T),
    Enum(String, String, T),
    Class(String, BamlMap<String, BamlValueWithMeta<T>>, T),
    Null(T),
}

impl<T> BamlValueWithMeta<T> {
    pub fn r#type(&self) -> String {
        let plain_value: BamlValue = self.into();
        plain_value.r#type()
    }

    /// Iterating over a `BamlValueWithMeta` produces a depth-first traversal
    /// of the value and all its children.
    pub fn iter(&self) -> BamlValueWithMetaIterator<T> {
        BamlValueWithMetaIterator::new(self)
    }

    pub fn value(self) -> BamlValue {
        match self {
            BamlValueWithMeta::String(v, _) => BamlValue::String(v),
            BamlValueWithMeta::Int(v, _) => BamlValue::Int(v),
            BamlValueWithMeta::Float(v, _) => BamlValue::Float(v),
            BamlValueWithMeta::Bool(v, _) => BamlValue::Bool(v),
            BamlValueWithMeta::Map(v, _) => {
                BamlValue::Map(v.into_iter().map(|(k, v)| (k, v.value())).collect())
            }
            BamlValueWithMeta::List(v, _) => {
                BamlValue::List(v.into_iter().map(|v| v.value()).collect())
            }
            BamlValueWithMeta::Media(v, _) => BamlValue::Media(v),
            BamlValueWithMeta::Enum(v, w, _) => BamlValue::Enum(v, w),
            BamlValueWithMeta::Class(n, fs, _) => {
                BamlValue::Class(n, fs.into_iter().map(|(k, v)| (k, v.value())).collect())
            }
            BamlValueWithMeta::Null(_) => BamlValue::Null,
        }
    }

    pub fn meta(&self) -> &T {
        match self {
            BamlValueWithMeta::String(_, m) => m,
            BamlValueWithMeta::Int(_, m) => m,
            BamlValueWithMeta::Float(_, m) => m,
            BamlValueWithMeta::Bool(_, m) => m,
            BamlValueWithMeta::Map(_, m) => m,
            BamlValueWithMeta::List(_, m) => m,
            BamlValueWithMeta::Media(_, m) => m,
            BamlValueWithMeta::Enum(_, _, m) => m,
            BamlValueWithMeta::Class(_, _, m) => m,
            BamlValueWithMeta::Null(m) => m,
        }
    }

    pub fn meta_mut(&mut self) -> &mut T {
        match self {
            BamlValueWithMeta::String(_, m) => m,
            BamlValueWithMeta::Int(_, m) => m,
            BamlValueWithMeta::Float(_, m) => m,
            BamlValueWithMeta::Bool(_, m) => m,
            BamlValueWithMeta::Map(_, m) => m,
            BamlValueWithMeta::List(_, m) => m,
            BamlValueWithMeta::Media(_, m) => m,
            BamlValueWithMeta::Enum(_, _, m) => m,
            BamlValueWithMeta::Class(_, _, m) => m,
            BamlValueWithMeta::Null(m) => m,
        }
    }

    pub fn with_default_meta(value: &BamlValue) -> BamlValueWithMeta<T>
    where
        T: Default,
    {
        use BamlValueWithMeta::*;
        match value {
            BamlValue::String(s) => String(s.clone(), T::default()),
            BamlValue::Int(i) => Int(*i, T::default()),
            BamlValue::Float(f) => Float(*f, T::default()),
            BamlValue::Bool(b) => Bool(*b, T::default()),
            BamlValue::Map(entries) => BamlValueWithMeta::Map(
                entries
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::with_default_meta(v)))
                    .collect(),
                T::default(),
            ),
            BamlValue::List(items) => List(
                items.iter().map(|i| Self::with_default_meta(i)).collect(),
                T::default(),
            ),
            BamlValue::Media(m) => Media(m.clone(), T::default()),
            BamlValue::Enum(n, v) => Enum(n.clone(), v.clone(), T::default()),
            BamlValue::Class(_, items) => Map(
                items
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::with_default_meta(v)))
                    .collect(),
                T::default(),
            ),
            BamlValue::Null => Null(T::default()),
        }
    }

    pub fn map_meta<'a, F, U>(&'a self, f: F) -> BamlValueWithMeta<U>
    where
        F: Fn(&'a T) -> U + Copy,
    {
        match self {
            BamlValueWithMeta::String(v, m) => BamlValueWithMeta::String(v.clone(), f(m)),
            BamlValueWithMeta::Int(v, m) => BamlValueWithMeta::Int(*v, f(m)),
            BamlValueWithMeta::Float(v, m) => BamlValueWithMeta::Float(*v, f(m)),
            BamlValueWithMeta::Bool(v, m) => BamlValueWithMeta::Bool(*v, f(m)),
            BamlValueWithMeta::Map(v, m) => BamlValueWithMeta::Map(
                v.iter().map(|(k, v)| (k.clone(), v.map_meta(f))).collect(),
                f(m),
            ),
            BamlValueWithMeta::List(v, m) => {
                BamlValueWithMeta::List(v.iter().map(|v| v.map_meta(f)).collect(), f(m))
            }
            BamlValueWithMeta::Media(v, m) => BamlValueWithMeta::Media(v.clone(), f(m)),
            BamlValueWithMeta::Enum(v, e, m) => BamlValueWithMeta::Enum(v.clone(), e.clone(), f(m)),
            BamlValueWithMeta::Class(n, fs, m) => BamlValueWithMeta::Class(
                n.clone(),
                fs.into_iter()
                    .map(|(k, v)| (k.clone(), v.map_meta(f)))
                    .collect(),
                f(m),
            ),
            BamlValueWithMeta::Null(m) => BamlValueWithMeta::Null(f(m)),
        }
    }

    pub fn map_meta_owned<F, U>(self, f: F) -> BamlValueWithMeta<U>
    where
        F: Fn(T) -> U + Copy,
    {
        match self {
            BamlValueWithMeta::String(v, m) => BamlValueWithMeta::String(v, f(m)),
            BamlValueWithMeta::Int(v, m) => BamlValueWithMeta::Int(v, f(m)),
            BamlValueWithMeta::Float(v, m) => BamlValueWithMeta::Float(v, f(m)),
            BamlValueWithMeta::Bool(v, m) => BamlValueWithMeta::Bool(v, f(m)),
            BamlValueWithMeta::Map(v, m) => BamlValueWithMeta::Map(
                v.into_iter().map(|(k, v)| (k, v.map_meta_owned(f))).collect(),
                f(m),
            ),
            BamlValueWithMeta::List(v, m) => {
                BamlValueWithMeta::List(v.into_iter().map(|v| v.map_meta_owned(f)).collect(), f(m))
            }
            BamlValueWithMeta::Media(v, m) => BamlValueWithMeta::Media(v, f(m)),
            BamlValueWithMeta::Enum(v, e, m) => BamlValueWithMeta::Enum(v, e, f(m)),
            BamlValueWithMeta::Class(n, fs, m) => BamlValueWithMeta::Class(
                n,
                fs.into_iter()
                    .map(|(k, v)| (k, v.map_meta_owned(f)))
                    .collect(),
                f(m),
            ),
            BamlValueWithMeta::Null(m) => BamlValueWithMeta::Null(f(m)),
        }
    }

    /// Combine two similar shaped baml values by tupling their metadata
    /// on a node-by-node basis.
    /// 
    /// The baml value calling `zip_meta` is the "primary" one, whose value
    /// data will live on in the returned baml value.
    pub fn zip_meta<U: Clone + std::fmt::Debug>(self, other: &BamlValueWithMeta<U>) -> Result<BamlValueWithMeta<(T,U)>>
    where T: std::fmt::Debug
    {
        let other_meta: U = other.meta().clone();
        let error_msg = String::new(); // format!("Could not unify {:?} with {:?}.", self, other);
        let ret = match (self, other) {
            (BamlValueWithMeta::Null(meta1), _) => {
                Result::<_,_>::Ok(BamlValueWithMeta::Null((meta1, other_meta)))
            },
            (BamlValueWithMeta::String(s1, meta1), BamlValueWithMeta::String(_s2, _)) if true => Ok(BamlValueWithMeta::String(s1, (meta1, other_meta))),
            (BamlValueWithMeta::String(_,_), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::Int(s1, meta1), BamlValueWithMeta::Int(_s2, _)) if true => Ok(BamlValueWithMeta::Int(s1, (meta1, other_meta))),
            (BamlValueWithMeta::Int(_,_), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::Float(s1, meta1), BamlValueWithMeta::Float(_s2, _)) if true => Ok(BamlValueWithMeta::Float(s1, (meta1, other_meta))),
            (BamlValueWithMeta::Float(_,_), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::Bool(s1, meta1), BamlValueWithMeta::Bool(_s2, _)) if true => Ok(BamlValueWithMeta::Bool(s1, (meta1, other_meta))),
            (BamlValueWithMeta::Bool(_,_), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::Map(s1, meta1), BamlValueWithMeta::Map(s2, _)) => {
                let map_result = s1.into_iter().zip(s2).map(|((k1,v1), (_k2,v2))| {
                    v1.zip_meta(v2).map(|res| (k1, res))
                }).collect::<Result<IndexMap<_, _>>>()?;
                Ok(BamlValueWithMeta::Map(map_result, (meta1, other_meta)))
            },
            (BamlValueWithMeta::Map(_,_), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::List(l1, meta1), BamlValueWithMeta::List(l2, _)) => {
                let list_result = l1.into_iter().zip(l2).map(|(item1, item2)| {
                    item1.zip_meta(item2)
                }).collect::<Result<Vec<_>>>()?;
                Ok( BamlValueWithMeta::List(list_result, (meta1, other_meta)))
                
            }
            (BamlValueWithMeta::List(_,_), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::Media(m1, meta1), BamlValueWithMeta::Media(_m2, _)) if true => {
                Ok(BamlValueWithMeta::Media(m1, (meta1, other_meta)))
            }
            (BamlValueWithMeta::Media(_, _), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::Enum(x1, y1, meta1), BamlValueWithMeta::Enum(_x2, _y2, _)) if true => {
                Ok(BamlValueWithMeta::Enum(x1, y1, (meta1, other_meta)))
            }
            (BamlValueWithMeta::Enum(_, _, _), _) => anyhow::bail!("Unification error"),
            (BamlValueWithMeta::Class(name1, fields1, meta1), BamlValueWithMeta::Class(_name2, fields2, _)) if true => {
                // TODO: We can remove a `clone` by checking that the fields
                // are ordered the same way between the two classes, then consuming
                // both classs' fields in parallel.
                // let map_result = fields1.into_iter().zip(fields2).map(|((k1,v1),(_k2,v2))| {
                //     v1.zip_meta(v2).map(|r| (k1, r))
                // }).collect::<Result<IndexMap<_,_>>>()?;
                let map_result = fields1.into_iter().map(|(k1, v1)| {
                    let v2 = fields2.get(&k1).context("Missing expected key")?;
                    v1.zip_meta(v2).map(|r| (k1, r))
                }).collect::<Result<IndexMap<_,_>>>()?;
                Ok(BamlValueWithMeta::Class(name1, map_result, (meta1, other_meta)))
            }
            (BamlValueWithMeta::Class(_, _, _), _) => anyhow::bail!("Unification error"),
        };
        ret.map_err(|_: anyhow::Error| anyhow::anyhow!(error_msg))
    }
}

/// An iterator over a BamlValue and all of its sub-values.
/// It yields entries in depth-first order.
pub struct BamlValueWithMetaIterator<'a, T> {
    stack: VecDeque<&'a BamlValueWithMeta<T>>,
}

impl<'a, T> BamlValueWithMetaIterator<'a, T> {
    /// Construct a new iterator. Users should do this via
    /// `.iter()` on a `BamlValueWithMeta` value.
    fn new(root: &'a BamlValueWithMeta<T>) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(root);
        BamlValueWithMetaIterator { stack }
    }
}

impl<'a, T: 'a> Iterator for BamlValueWithMetaIterator<'a, T> {
    type Item = &'a BamlValueWithMeta<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.stack.pop_back() {
            // Get all the children and push them onto the stack.
            match value {
                BamlValueWithMeta::List(items, _) => {
                    self.stack.extend(items);
                }
                BamlValueWithMeta::Map(fields, _) => {
                    for (_, v) in fields.iter() {
                        self.stack.push_back(v);
                    }
                }
                BamlValueWithMeta::Class(_, fields, _) => {
                    for (_, v) in fields.iter() {
                        self.stack.push_back(v);
                    }
                }
                // These items have to children.
                BamlValueWithMeta::String(..)
                | BamlValueWithMeta::Int(..)
                | BamlValueWithMeta::Float(..)
                | BamlValueWithMeta::Bool(..)
                | BamlValueWithMeta::Media(..)
                | BamlValueWithMeta::Enum(..)
                | BamlValueWithMeta::Null(..) => {}
            }
            Some(value)
        } else {
            None
        }
    }
}

// Boilerplate.
impl<'a, T: 'a> IntoIterator for &'a BamlValueWithMeta<T> {
    type Item = &'a BamlValueWithMeta<T>;
    type IntoIter = BamlValueWithMetaIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> From<&BamlValueWithMeta<T>> for BamlValue {
    fn from(baml_value: &BamlValueWithMeta<T>) -> BamlValue {
        use BamlValueWithMeta::*;
        match baml_value {
            String(v, _) => BamlValue::String(v.clone()),
            Int(v, _) => BamlValue::Int(*v),
            Float(v, _) => BamlValue::Float(*v),
            Bool(v, _) => BamlValue::Bool(*v),
            Map(v, _) => {
                BamlValue::Map(v.into_iter().map(|(k, v)| (k.clone(), v.into())).collect())
            }
            List(v, _) => BamlValue::List(v.iter().map(|v| v.into()).collect()),
            Media(v, _) => BamlValue::Media(v.clone()),
            Enum(enum_name, v, _) => BamlValue::Enum(enum_name.clone(), v.clone()),
            Class(class_name, v, _) => BamlValue::Class(
                class_name.clone(),
                v.into_iter().map(|(k, v)| (k.clone(), v.into())).collect(),
            ),
            Null(_) => BamlValue::Null,
        }
    }
}

impl<T> From<BamlValueWithMeta<T>> for BamlValue {
    fn from(baml_value: BamlValueWithMeta<T>) -> BamlValue {
        use BamlValueWithMeta::*;
        match baml_value {
            String(v, _) => BamlValue::String(v),
            Int(v, _) => BamlValue::Int(v),
            Float(v, _) => BamlValue::Float(v),
            Bool(v, _) => BamlValue::Bool(v),
            Map(v, _) => BamlValue::Map(v.into_iter().map(|(k, v)| (k, v.into())).collect()),
            List(v, _) => BamlValue::List(v.into_iter().map(|v| v.into()).collect()),
            Media(v, _) => BamlValue::Media(v),
            Enum(enum_name, v, _) => BamlValue::Enum(enum_name, v),
            Class(class_name, v, _) => BamlValue::Class(
                class_name,
                v.into_iter().map(|(k, v)| (k, v.into())).collect(),
            ),
            Null(_) => BamlValue::Null,
        }
    }
}

/// This special-purpose serializer is used for jinja.
/// When we want to extend the orchestrator with BamlValues packing more
/// metadata than just a `Vec<ResponseCheck>`, `
impl Serialize for BamlValueWithMeta<Vec<ResponseCheck>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            BamlValueWithMeta::String(v, cr) => serialize_with_checks(v, cr, serializer),
            BamlValueWithMeta::Int(v, cr) => serialize_with_checks(v, cr, serializer),
            BamlValueWithMeta::Float(v, cr) => serialize_with_checks(v, cr, serializer),
            BamlValueWithMeta::Bool(v, cr) => serialize_with_checks(v, cr, serializer),
            BamlValueWithMeta::Map(v, cr) => {
                let mut map = serializer.serialize_map(None)?;
                for (key, value) in v {
                    map.serialize_entry(key, value)?;
                }
                add_checks(&mut map, cr)?;
                map.end()
            }
            BamlValueWithMeta::List(v, cr) => serialize_with_checks(v, cr, serializer),
            BamlValueWithMeta::Media(v, cr) => serialize_with_checks(v, cr, serializer),
            BamlValueWithMeta::Enum(_enum_name, v, cr) => serialize_with_checks(v, cr, serializer),
            BamlValueWithMeta::Class(_class_name, v, cr) => {
                if cr.is_empty() {
                    let mut map = serializer.serialize_map(None)?;
                    for (key, value) in v {
                        map.serialize_entry(key, value)?;
                    }
                    add_checks(&mut map, cr)?;
                    map.end()
                } else {
                    let mut checked_value = serializer.serialize_map(Some(2))?;
                    checked_value.serialize_entry("value", &v)?;
                    add_checks(&mut checked_value, cr)?;
                    checked_value.end()
                }
            }
            BamlValueWithMeta::Null(cr) => serialize_with_checks(&(), cr, serializer),
        }
    }
}

fn serialize_with_checks<S, T: Serialize>(
    value: &T,
    checks: &[ResponseCheck],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if !checks.is_empty() {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("value", value)?;
        add_checks(&mut map, checks)?;
        map.end()
    } else {
        value.serialize(serializer)
    }
}

fn add_checks<'a, S: SerializeMap>(
    map: &'a mut S,
    checks: &'a [ResponseCheck],
) -> Result<(), S::Error>
{
    if !checks.is_empty() {
        let checks_map: BamlMap<_,_> = checks.iter().map(|check| (check.name.clone(), check)).collect();
        map.serialize_entry("checks", &checks_map)?;
    }
    Ok(())
}

/// This type is used in `BamlResponseValue` to summarize data about the
/// completion state and completion behavior of a BamlValueWithMeta node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Completion {
    pub state: CompletionState,
    pub display: bool,
    pub required_done: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum CompletionState {
    Pending,
    Incomplete,
    Complete,
}

impl Default for Completion {
    fn default() -> Self {
        Completion {
            state: CompletionState::Complete,
            display: false,
            required_done: false,
        }
    }
}