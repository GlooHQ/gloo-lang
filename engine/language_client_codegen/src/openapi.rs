use std::path::PathBuf;

use anyhow::{Context, Result};
use baml_types::{BamlMediaType, FieldType, LiteralValue, TypeValue};
use indexmap::IndexMap;
use internal_baml_core::ir::{
    repr::{Function, IntermediateRepr, Node, Walker},
    ClassWalker, EnumWalker,
};
use serde::Serialize;
use serde_json::json;

use crate::{
    dir_writer::{FileCollector, LanguageFeatures, RemoveDirBehavior},
    field_type_attributes, TypeCheckAttributes,
};

#[derive(Default)]
pub(super) struct OpenApiLanguageFeatures {}

impl LanguageFeatures for OpenApiLanguageFeatures {
    const CONTENT_PREFIX: &'static str = r#"
###############################################################################
#
#  Welcome to Baml! To use this generated code, please run the following:
#
#  $ openapi-generator generate -i openapi.yaml -g <language> -o <output_dir>
#
###############################################################################

# This file was generated by BAML: please do not edit it. Instead, edit the
# BAML files and re-generate this code.

        "#;

    const REMOVE_DIR_BEHAVIOR: RemoveDirBehavior = RemoveDirBehavior::Unsafe;

    const GITIGNORE: Option<&'static str> = Some(
        r#"
###############################################################################
#
#  Welcome to Baml!
#
#  This .gitignore is here to keep you from accidentally checking in your
#  generated OpenAPI client - we strongly suggest you generate it at build time
#  instead.
#
#  Do not edit this file! BAML will overwrite all changes to this file.
#
#  If you do need to edit it, let us know: https://docs.boundaryml.com/contact
#
###############################################################################

# Ignore everything in this dir (because it's autogenerated)
*

# Except this .gitignore file
!.gitignore

# Preserving changes to .openapi-generator-ignore is also important
!.openapi-generator-ignore
"#,
    );
}

pub struct OpenApiSchema<'ir> {
    paths: Vec<OpenApiMethodDef<'ir>>,
    schemas: IndexMap<&'ir str, TypeSpecWithMeta>,
}

impl Serialize for OpenApiSchema<'_> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> core::result::Result<S::Ok, S::Error> {
        let schemas = match self
            .schemas
            .iter()
            .map(|(name, schema)| Ok((*name, serde_json::to_value(schema)?)))
            .collect::<core::result::Result<Vec<_>, serde_json::Error>>()
        {
            Ok(schemas) => schemas,
            Err(e) => return Err(serde::ser::Error::custom(e)),
        };
        json!({
            "openapi": "3.0.0",
            "info": {
                "description": "baml-cli serve",
                "version": "0.1.0",
                "title": "baml-cli serve",
            },
            "servers": [
                {
                    "url": "{address}",
                    "variables": {
                        "address": {
                            // Super important! This should NOT have trailing slashes!!!
                            "default": "http://localhost:2024",
                        },
                    },
                },
            ],
            "paths": self.paths
                .iter()
                .flat_map(|p| vec![
                    (format!("/call/{}", p.function_name), p.as_json("call", "application/json")),
                    // TODO: publish a schema for streams
                    // (format!("/stream/{}", p.function_name), p.as_json("stream", "text/event-stream"))
                ])
                .collect::<IndexMap<_, _>>(),
            "components": {
                "requestBodies": self.paths.iter().map(|p| {
                    (p.function_name, json!({
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": p.request_body
                            }
                        }
                    }))
                }).collect::<IndexMap<_, _>>(),
                "schemas": vec![
                    (
                        "BamlImage",
                        json!({
                            "oneOf": [
                                {
                                    "type": "object",
                                    "title": "BamlImageBase64",
                                    "properties": {
                                        "base64": {
                                            "type": "string"
                                        },
                                        "media_type": {
                                            "type": "string",
                                        },
                                    },
                                    "required": ["base64"],
                                },
                                {
                                    "type": "object",
                                    "title": "BamlImageUrl",
                                    "properties": {
                                        "url": {
                                            "type": "string",
                                        },
                                        "media_type": {
                                            "type": "string",
                                        },
                                    },
                                    "required": ["url"],
                                }
                            ],
                        }),
                    ),
                    (
                        "BamlAudio",
                        json!({
                            "oneOf": [
                                {
                                    "type": "object",
                                    "title": "BamlAudioBase64",
                                    "properties": {
                                        "base64": {
                                            "type": "string",
                                        },
                                        "media_type": {
                                            "type": "string",
                                        },
                                    },
                                    "required": ["base64"],
                                },
                                {
                                    "type": "object",
                                    "title": "BamlAudioUrl",
                                    "properties": {
                                        "url": {
                                            "type": "string",
                                        },
                                        "media_type": {
                                            "type": "string",
                                        },
                                    },
                                    "required": ["url"],
                                }
                            ],
                        }),
                    ),
                    (
                        "BamlOptions",
                        json!({
                            "type": "object",
                            "nullable": false,
                            "properties": {
                                "client_registry": {
                                    "type": "object",
                                    "nullable": false,
                                    "properties": {
                                        "clients": {
                                            "type": "array",
                                            "items": {
                                                "$ref": "#/components/schemas/ClientProperty"
                                            }
                                        },
                                        "primary": {
                                            "type": "string",
                                            "nullable": false
                                        }
                                    },
                                    "required": ["clients"]
                                }
                            }
                        })
                    ),
                    (
                        "ClientProperty",
                        json!({
                            "type": "object",
                            "properties": {
                                "name": {
                                    "type": "string"
                                },
                                "provider": {
                                    "type": "string"
                                },
                                "retry_policy": {
                                    "type": "string",
                                    "nullable": false
                                },
                                "options": {
                                    "type": "object",
                                    "additionalProperties": true
                                }
                            },
                            "required": ["name", "provider", "options"]
                        })
                    ),
                    (  "Check",
                        json!({
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "expr": { "type": "string" },
                                "status": { "type": "string" }
                            }

                        })
                    )
                ]
                .into_iter()
                .chain(schemas.into_iter())
                .collect::<IndexMap<_, _>>(),
            }
        })
        .serialize(serializer)
    }
}

struct OpenApiMethodDef<'ir> {
    function_name: &'ir str,
    request_body: TypeSpecWithMeta,
    response: TypeSpecWithMeta,
}

impl OpenApiMethodDef<'_> {
    fn as_json(&self, tag: &str, response_type: &str) -> serde_json::Value {
        let mut as_json = json!({
            "requestBody": {
                "$ref": format!("#/components/requestBodies/{}", self.function_name),
            },
            "responses": {
                "200": {
                    "description": "Successful operation",
                    "content": {
                        response_type: {
                            "schema": self.response
                        }
                    }
                }
            }
        });

        as_json.as_object_mut().unwrap().extend(
            if tag == "call" {
                json!({
                    // "tags": vec!["call"],
                    "operationId": self.function_name,
                })
            } else {
                json!({
                    "tags": ["stream"],
                    // "operationId": self.function_name,
                })
            }
            .as_object_mut()
            .unwrap()
            .clone(),
        );

        json!({
            "post": as_json
        })
    }
}

pub(crate) fn generate(
    ir: &IntermediateRepr,
    generator: &crate::GeneratorArgs,
) -> Result<IndexMap<PathBuf, String>> {
    let mut collector = FileCollector::<OpenApiLanguageFeatures>::new();

    let schema: OpenApiSchema = (ir, generator).try_into()?;

    collector.add_file("openapi.yaml", serde_yaml::to_string(&schema)?);
    collector.add_file(
        ".openapi-generator-ignore",
        r#"
.gitignore
"#,
    );

    collector.commit(&generator.output_dir())
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'_ crate::GeneratorArgs)> for OpenApiSchema<'ir> {
    type Error = anyhow::Error;

    fn try_from((ir, _): (&'ir IntermediateRepr, &'_ crate::GeneratorArgs)) -> Result<Self> {
        Ok(Self {
            paths: ir
                .walk_functions()
                .map(|f| {
                    f.try_into().context(format!(
                        "Failed to convert BAML function {} to OpenAPI method",
                        f.item.elem.name()
                    ))
                })
                .collect::<Result<_>>()?,
            schemas: vec![]
                .into_iter()
                .chain(ir.walk_enums().map(|e| Ok((e.name(), e.try_into()?))))
                .chain(ir.walk_classes().map(|c| Ok((c.name(), c.try_into()?))))
                .collect::<Result<_>>()?,
        })
    }
}

fn check() -> TypeSpecWithMeta {
    TypeSpecWithMeta {
        meta: TypeMetadata::default(),
        type_spec: TypeSpec::Ref {
            r#ref: "#components/schemas/Check".to_string(),
        },
    }
}

/// The type definition for a single "Checked_*" type. Note that we don't
/// produce a named type for each of these the way we do for SDK
/// codegeneration.
fn type_def_for_checks(checks: TypeCheckAttributes) -> TypeSpecWithMeta {
    TypeSpecWithMeta {
        meta: TypeMetadata::default(),
        type_spec: TypeSpec::Inline(TypeDef::Class {
            properties: checks
                .0
                .iter()
                .map(|check_name| (check_name.clone(), check()))
                .collect(),
            required: checks.0.into_iter().collect(),
            additional_properties: false,
        }),
    }
}

impl<'ir> TryFrom<Walker<'ir, &'ir Node<Function>>> for OpenApiMethodDef<'ir> {
    type Error = anyhow::Error;

    fn try_from(value: Walker<'ir, &'ir Node<Function>>) -> Result<Self> {
        let function_name = value.item.elem.name();
        let mut properties: IndexMap<String, TypeSpecWithMeta> = value
                        .item
                        .elem
                        .inputs()
                        .iter()
                        .map(|(name, t)| {
                            Ok((
                                name.to_string(),
                                t.to_type_spec(value.db).context(format!(
                                    "Failed to convert arg {name} (for function {function_name}) to OpenAPI type",
                                ))?,
                            ))
                        })
                        .collect::<Result<_>>()?;
        properties.insert(
            "__baml_options__".to_string(),
            TypeSpecWithMeta {
                meta: TypeMetadata {
                    title: None,
                    r#enum: None,
                    r#const: None,
                    nullable: true,
                },
                type_spec: TypeSpec::Ref {
                    r#ref: "#/components/schemas/BamlOptions".into(),
                },
            },
        );
        Ok(Self {
            function_name,
            request_body: TypeSpecWithMeta {
                meta: TypeMetadata {
                    // We _deliberately_ set this, even though OpenAPI doesn't require it,
                    // because some generators will de-duplicate names of generated types
                    // based on type shape
                    //
                    // For example, the Golang generator will use "ClassifyMessageRequest" as the
                    // request type for b.GetOrderInfo if they both have (input: string) as their
                    // function arg signature (I think the Java generator too?)
                    //
                    // title: None,
                    title: Some(format!("{}Request", function_name)),
                    r#enum: None,
                    r#const: None,
                    nullable: false,
                },
                type_spec: TypeSpec::Inline(TypeDef::Class {
                    properties,
                    required: value
                        .item
                        .elem
                        .inputs()
                        .iter()
                        .filter_map(|(name, t)| {
                            if t.is_optional() {
                                None
                            } else {
                                Some(name.to_string())
                            }
                        })
                        .collect(),
                    additional_properties: false,
                }),
            },
            response: {
                let mut response_type = value.item.elem.output().to_type_spec(value.db)?;
                response_type.meta.title = Some(format!("{}Response", function_name));
                response_type
            },
        })
    }
}

impl<'ir> TryFrom<EnumWalker<'ir>> for TypeSpecWithMeta {
    type Error = anyhow::Error;

    fn try_from(e: EnumWalker<'ir>) -> Result<Self> {
        Ok(TypeSpecWithMeta {
            meta: TypeMetadata {
                title: None,
                r#enum: Some(
                    e.item
                        .elem
                        .values
                        .iter()
                        .map(|v| v.0.elem.0.to_string())
                        .collect(),
                ),
                r#const: None,
                nullable: false,
            },
            type_spec: TypeSpec::Inline(TypeDef::String),
        })
    }
}

impl<'ir> TryFrom<ClassWalker<'ir>> for TypeSpecWithMeta {
    type Error = anyhow::Error;

    fn try_from(c: ClassWalker<'ir>) -> Result<Self> {
        Ok(TypeSpecWithMeta {
            meta: TypeMetadata {
                title: None,
                r#enum: None,
                r#const: None,
                nullable: false,
            },
            type_spec: TypeSpec::Inline(TypeDef::Class {
                properties: c
                    .item
                    .elem
                    // TODO: should go through walk_fields()
                    .static_fields
                    .iter()
                    .map(|f| {
                        Ok((
                            f.elem.name.to_string(),
                            f.elem.r#type.elem.to_type_spec(c.db).context(format!(
                                "Failed to convert {}.{} to OpenAPI type",
                                c.name(),
                                f.elem.name
                            ))?,
                        ))
                    })
                    .collect::<Result<_>>()?,
                required: c
                    .item
                    .elem
                    .static_fields
                    .iter()
                    .filter_map(|f| {
                        if f.elem.r#type.elem.is_optional() {
                            None
                        } else {
                            Some(f.elem.name.to_string())
                        }
                    })
                    .collect(),
                additional_properties: false,
            }),
        })
    }
}

trait ToTypeReferenceInTypeDefinition<'ir> {
    fn to_type_spec(&self, ir: &'ir IntermediateRepr) -> Result<TypeSpecWithMeta>;
}

impl<'ir> ToTypeReferenceInTypeDefinition<'ir> for FieldType {
    fn to_type_spec(&self, _ir: &'ir IntermediateRepr) -> Result<TypeSpecWithMeta> {
        Ok(match self {
            FieldType::Enum(name) | FieldType::Class(name) => TypeSpecWithMeta {
                meta: TypeMetadata {
                    title: None,
                    r#enum: None,
                    r#const: None,
                    nullable: false,
                },
                type_spec: TypeSpec::Ref {
                    r#ref: format!("#/components/schemas/{}", name),
                },
            },
            FieldType::RecursiveTypeAlias(_) => TypeSpecWithMeta {
                meta: TypeMetadata {
                    title: None,
                    r#enum: None,
                    r#const: None,
                    nullable: false,
                },
                type_spec: TypeSpec::AnyValue { any_of: vec![] },
            },
            FieldType::Literal(v) => TypeSpecWithMeta {
                meta: TypeMetadata {
                    title: None,
                    r#enum: None,
                    r#const: None,
                    nullable: false,
                },
                type_spec: match v {
                    LiteralValue::Int(_) => TypeSpec::Inline(TypeDef::Int),
                    LiteralValue::Bool(_) => TypeSpec::Inline(TypeDef::Bool),
                    LiteralValue::String(_) => TypeSpec::Inline(TypeDef::String),
                },
            },
            FieldType::List(inner) => TypeSpecWithMeta {
                meta: TypeMetadata {
                    title: None,
                    r#enum: None,
                    r#const: None,
                    nullable: false,
                },
                type_spec: TypeSpec::Inline(TypeDef::Array {
                    items: inner.to_type_spec(_ir)?.into(),
                }),
            },
            FieldType::Map(key, value) => {
                if !matches!(**key, FieldType::Primitive(TypeValue::String)) {
                    anyhow::bail!("BAML<->OpenAPI only supports string keys in maps")
                }
                TypeSpecWithMeta {
                    meta: TypeMetadata {
                        title: None,
                        r#enum: None,
                        r#const: None,
                        nullable: false,
                    },
                    type_spec: TypeSpec::Inline(TypeDef::Map {
                        additional_properties: Box::new(value.to_type_spec(_ir)?),
                    }),
                }
            }
            FieldType::Primitive(inner) => TypeSpecWithMeta {
                meta: TypeMetadata {
                    title: None,
                    r#enum: None,
                    r#const: None,
                    nullable: false,
                },
                type_spec: match inner {
                    TypeValue::Bool => TypeSpec::Inline(TypeDef::Bool),
                    // TODO: should this support "format: double"?
                    TypeValue::Float => TypeSpec::Inline(TypeDef::Float),
                    // TODO: should this support "format: int64"?
                    TypeValue::Int => TypeSpec::Inline(TypeDef::Int),
                    TypeValue::Null => anyhow::bail!(
                        "BAML<->OpenAPI only allows nulls in unions, not as a literal"
                    ),
                    TypeValue::String => TypeSpec::Inline(TypeDef::String),
                    TypeValue::Media(BamlMediaType::Audio) => TypeSpec::Ref {
                        r#ref: "#/components/schemas/BamlAudio".to_string(),
                    },
                    TypeValue::Media(BamlMediaType::Image) => TypeSpec::Ref {
                        r#ref: "#/components/schemas/BamlImage".to_string(),
                    },
                },
            },
            FieldType::Union(union) => {
                let (_nulls, nonnull_types): (Vec<_>, Vec<_>) =
                    union.iter().partition(|t| t.is_null());

                let one_of = nonnull_types
                    .iter()
                    .map(|t| t.to_type_spec(_ir))
                    .collect::<Result<Vec<_>>>()?;

                if one_of.is_empty() {
                    anyhow::bail!("BAML<->OpenAPI unions must have at least one non-null type")
                }

                TypeSpecWithMeta {
                    meta: TypeMetadata {
                        title: None,
                        r#enum: None,
                        r#const: None,
                        nullable: false,
                    },
                    type_spec: TypeSpec::Union { one_of },
                }
            }
            FieldType::Tuple(_) => {
                anyhow::bail!("BAML<->OpenAPI tuple support is not implemented")
            }
            FieldType::Optional(inner) => {
                // TODO: if type_spec is of an enum, consider adding "null" to the list of values
                // something i saw suggested doing this
                inner.to_type_spec(_ir)?
            }
            FieldType::Constrained { base, .. } => match field_type_attributes(self) {
                Some(checks) => {
                    let base_type_ref = base.to_type_spec(_ir)?;
                    let checks_type_spec = type_def_for_checks(checks);
                    TypeSpecWithMeta {
                        meta: TypeMetadata::default(),
                        type_spec: TypeSpec::Inline(TypeDef::Class {
                            properties: vec![
                                ("value".to_string(), base_type_ref),
                                ("checks".to_string(), checks_type_spec),
                            ]
                            .into_iter()
                            .collect(),
                            required: vec!["value".to_string(), "checks".to_string()],
                            additional_properties: false,
                        }),
                    }
                }
                None => base.to_type_spec(_ir)?,
            },
        })
    }
}

#[derive(Clone, Debug, Serialize)]
struct TypeSpecWithMeta {
    #[serde(flatten)]
    meta: TypeMetadata,

    #[serde(flatten)]
    type_spec: TypeSpec,
}

#[derive(Clone, Debug, Serialize, Default)]
struct TypeMetadata {
    /// Pydantic includes this by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    /// JSON schema considers 'enum' to be a validation rule, not a type,
    /// so it can be attached to any type.
    /// We only allow string-shaped enums
    #[serde(skip_serializing_if = "Option::is_none")]
    r#enum: Option<Vec<String>>,

    /// We only allow string-shaped const values
    #[serde(skip_serializing_if = "Option::is_none")]
    r#const: Option<String>,
    // description: Option<String>,
    /// Nulls in OpenAPI are weird: https://swagger.io/docs/specification/data-models/data-types/
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    nullable: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
enum TypeSpec {
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
    },
    Inline(TypeDef),
    // In OpenAPI, "the value must be a single type and not an array of types"
    // so we do not need to support type: Vec<>
    // InlineUnion(InlineUnion),
    Union {
        #[serde(rename = "oneOf", alias = "oneOf")]
        one_of: Vec<TypeSpecWithMeta>,
    },
    AnyValue {
        #[serde(rename = "anyOf", alias = "anyOf")]
        any_of: Vec<TypeSpecWithMeta>,
    },
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
enum TypeDef {
    #[serde(rename = "string")]
    String,

    #[serde(rename = "object")]
    #[serde(rename_all = "camelCase")]
    Class {
        properties: IndexMap<String, TypeSpecWithMeta>,
        required: Vec<String>,
        /// OpenAPI defaults this to true, but we want it to be false
        additional_properties: bool,
    },

    #[serde(rename = "object")]
    #[serde(rename_all = "camelCase")]
    Map {
        /// value type
        additional_properties: Box<TypeSpecWithMeta>,
    },

    #[serde(rename = "array")]
    Array { items: Box<TypeSpecWithMeta> },

    #[serde(rename = "integer")]
    Int,

    #[serde(rename = "number")]
    Float,

    #[serde(rename = "boolean")]
    Bool,
}
